# RepositoryRulePullRequestParameters

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**allowed_merge_methods** | Option<**Vec<String>**> | Array of allowed merge methods. Allowed values include `merge`, `squash`, and `rebase`. At least one option must be enabled. | [optional]
**dismiss_stale_reviews_on_push** | **bool** | New, reviewable commits pushed will dismiss previous pull request review approvals. | 
**require_code_owner_review** | **bool** | Require an approving review in pull requests that modify files that have a designated code owner. | 
**require_last_push_approval** | **bool** | Whether the most recent reviewable push must be approved by someone other than the person who pushed it. | 
**required_approving_review_count** | **i32** | The number of approving reviews that are required before a pull request can be merged. | 
**required_review_thread_resolution** | **bool** | All conversations on code must be resolved before a pull request can be merged. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


