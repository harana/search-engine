use std::collections::HashMap;

use openidconnect::{
    AccessTokenHash,
    AuthenticationFlow,
    AuthorizationCode,
    ClientId,
    ClientSecret,
    CsrfToken,
    IssuerUrl,
    Nonce,
    OAuth2TokenResponse,
    PkceCodeChallenge,
    RedirectUrl,
    Scope,
    TokenResponse,
};

use openidconnect::core::{
    CoreAuthenticationFlow,
    CoreClient,
    CoreProviderMetadata,
    CoreResponseType,
    CoreUserInfoClaims,
};

use harana_common::anyhow::Result;
use harana_common::lazy_static::lazy_static;
use harana_common::url;

use openidconnect::reqwest::http_client;
use url::Url;

#[derive(Debug)]
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

impl OpenIdManager {

    fn connect(provider: OpenIDProvider, client_id: String, client_secret: String, redirect_url: String) -> Option<String> {
        let issuer_url = OPENID_ISSUER_URLS.get(&provider);

        let provider_metadata = CoreProviderMetadata::discover(
            &IssuerUrl::new(issuer_url)?,
            http_client,
        )?;

        let client = CoreClient::from_provider_metadata(
            provider_metadata,
            ClientId::new(client_id),
            Some(ClientSecret::new(client_secret)),
        ).set_redirect_uri(RedirectUrl::new(redirect_url)?);

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

        auth_url
    }

    fn authorization_code(callback_url: String) -> Option<String> {
        Url::parse(&callback_url)
            .unwrap()
            .query_pairs()
            .find(|(key, _)| key == "code")
            .map(|(_, value)| value.into_owned())
    }

    fn claims(authorization_code: String) -> Option<CoreUserInfoClaims> {
        let token_response =
            client
                .exchange_code(AuthorizationCode::new("some authorization code".to_string()))
                .set_pkce_verifier(pkce_verifier)
                .request(http_client)?;

        let id_token = token_response
            .id_token()
            .ok_or_else(|| anyhow!("Server did not return an ID token"))?;
        let claims = id_token.claims(&client.id_token_verifier(), &nonce)?;

        client
            .user_info(token_response.access_token().to_owned(), None)
            .map_err(|err| anyhow!("No user info endpoint: {:?}", err))?
            .request(http_client)
            .map_err(|err| anyhow!("Failed requesting user info: {:?}", err))?
    }
}