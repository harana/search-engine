use std::collections::HashMap;

use oauth2::basic::{BasicErrorResponseType, BasicRevocationErrorResponse, BasicTokenType};
use oauth2::StandardRevocableToken;
use openidconnect::*;
use openidconnect::core::*;
use harana_common::anyhow::{anyhow, Result};
use harana_common::lazy_static::lazy_static;
use harana_common::url;

use openidconnect::reqwest::http_client;
use url::Url;

#[derive(Debug, Eq, Hash, PartialEq)]
pub enum OpenIDProvider {
    Apple,
    Auth0,
    Github,
    Gitlab,
    Google,
    Microsoft,
    Slack,
}

lazy_static! {
    pub static ref OPENID_ISSUER_URLS: HashMap<OpenIDProvider, &'static str> = {
        let mut m = HashMap::new();
        m.insert(OpenIDProvider::Apple, "https://appleid.apple.com");
        m.insert(OpenIDProvider::Auth0, "https://YOUR_AUTH0_DOMAIN");
        m.insert(OpenIDProvider::Github, "https://github.com");
        m.insert(OpenIDProvider::Gitlab, "https://gitlab.com");
        m.insert(OpenIDProvider::Google, "https://accounts.google.com");
        m.insert(OpenIDProvider::Microsoft, "https://login.microsoftonline.com/common/v2.0");
        m.insert(OpenIDProvider::Slack, "https://slack.com");
        m
    };
}

struct OpenIdConnection {
    auth_url: Url,
    client: Client<
        EmptyAdditionalClaims,
        CoreAuthDisplay,
        CoreGenderClaim,
        CoreJweContentEncryptionAlgorithm,
        CoreJwsSigningAlgorithm,
        CoreJsonWebKeyType,
        CoreJsonWebKeyUse,
        CoreJsonWebKey,
        CoreAuthPrompt,
        StandardErrorResponse<BasicErrorResponseType>,
        CoreTokenResponse,
        BasicTokenType,
        CoreTokenIntrospectionResponse,
        StandardRevocableToken,
        BasicRevocationErrorResponse
    >,
    csrf_token: CsrfToken,
    nonce: Nonce,
    pkce_verifier: PkceCodeVerifier
}

impl OpenIdConnection {

    pub fn new(provider: OpenIDProvider, client_id: String, client_secret: String, redirect_url: String) -> Self {
        let issuer_url = OPENID_ISSUER_URLS.get(&provider).unwrap().to_string();

        let provider_metadata = CoreProviderMetadata::discover(
            &IssuerUrl::new(issuer_url).unwrap(),
            http_client,
        ).unwrap();

        let client = CoreClient::from_provider_metadata(
            provider_metadata,
            ClientId::new(client_id),
            Some(ClientSecret::new(client_secret)),
        ).set_redirect_uri(RedirectUrl::new(redirect_url).unwrap());

        let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

        let (auth_url, csrf_token, nonce) = client
            .authorize_url(
                CoreAuthenticationFlow::AuthorizationCode,
                CsrfToken::new_random,
                Nonce::new_random,
            )
            .add_scope(Scope::new("read".to_string()))
            .add_scope(Scope::new("write".to_string()))
            .set_pkce_challenge(pkce_challenge)
            .url();

        Self {
            auth_url,
            client,
            csrf_token,
            nonce,
            pkce_verifier
        }
    }

    fn authorization_code(callback_url: String) -> Option<String> {
        Url::parse(&callback_url)
            .unwrap()
            .query_pairs()
            .find(|(key, _)| key == "code")
            .map(|(_, value)| value.into_owned())
    }

    fn claims(&self, authorization_code: String) -> Result<CoreUserInfoClaims> {
        let token_response =
            self.client
                .exchange_code(AuthorizationCode::new(authorization_code))
                .set_pkce_verifier(PkceCodeVerifier::new(self.pkce_verifier.secret().to_string()))
                .request(http_client)?;

        let id_token = token_response
            .id_token()
            .ok_or_else(|| anyhow!("Server did not return an ID token"))?;
        let claims = id_token.claims(&self.client.id_token_verifier(), &self.nonce)?;

        Ok(
            self.client
            .user_info(token_response.access_token().to_owned(), None)
            .map_err(|err| anyhow!("No user info endpoint: {:?}", err))?
            .request(http_client)
            .map_err(|err| anyhow!("Failed requesting user info: {:?}", err))?
        )
    }
}