// =================================================================
//
//                           * WARNING *
//
//                    This file is generated!
//
//  Changes made to this file will be overwritten. If changes are
//  required to the generated code, the service_crategen project
//  must be updated to generate the changes.
//
// =================================================================

use std::error::Error;
use std::fmt;
use std::io;

#[allow(warnings)]
use futures::future;
use futures::Future;
use rusoto_core::reactor::{CredentialsProvider, RequestDispatcher};
use rusoto_core::request::DispatchSignedRequest;
use rusoto_core::region;
use rusoto_core::{ClientInner, RusotoFuture};

use rusoto_core::request::HttpDispatchError;
use rusoto_core::credential::{CredentialsError, ProvideAwsCredentials};

use serde_json;
use rusoto_core::param::{Params, ServiceParams};
use rusoto_core::signature::SignedRequest;
use serde_json::from_str;
use serde_json::Value as SerdeJsonValue;
/// <p>Details about the application.</p>
#[derive(Default, Debug, Clone)]
pub struct Application {
    /// <p>The application Amazon Resource Name (ARN).</p>
    pub application_id: Option<String>,
    /// <p>The name of the author publishing the app.\nMin Length=1. Max Length=127.\nPattern &quot;^<a href="(%5Ba-z0-9%5D%7C-(?!-))*%5Ba-z0-9%5D">a-z0-9</a>?$&quot;;</p>
    pub author: Option<String>,
    /// <p>The date/time this resource was created.</p>
    pub creation_time: Option<String>,
    /// <p>The description of the application.\nMin Length=1. Max Length=256</p>
    pub description: Option<String>,
    /// <p>Labels to improve discovery of apps in search results.\nMin Length=1. Max Length=127. Maximum number of labels: 10\nPattern: &quot;^[a-zA-Z0-9+\-_:\/@]+$&quot;;</p>
    pub labels: Option<Vec<String>>,
    /// <p>A link to a license file of the app that matches the spdxLicenseID of your application.\nMax size 5 MB</p>
    pub license_url: Option<String>,
    /// <p>The name of the application.\nMin Length=1. Max Length=140\nPattern: &quot;[a-zA-Z0-9\-]+&quot;;</p>
    pub name: Option<String>,
    /// <p>A link to the Readme file that contains a more detailed description of the application and how it works in markdown language.\nMax size 5 MB</p>
    pub readme_url: Option<String>,
    /// <p>A valid identifier from https://spdx.org/licenses/.</p>
    pub spdx_license_id: Option<String>,
    /// <p>Version information about the application.</p>
    pub version: Option<Version>,
}

/// <p>List of application details.</p>
#[derive(Default, Debug, Clone)]
pub struct ApplicationPage {
    /// <p>Array of application summaries.</p>
    pub applications: Option<Vec<ApplicationSummary>>,
    /// <p>The token to request the next page of results.</p>
    pub next_token: Option<String>,
}

/// <p>Policy statements applied to the application.</p>
#[derive(Default, Debug, Clone)]
pub struct ApplicationPolicy {
    /// <p>Array of policy statements applied to the application.</p>
    pub statements: Option<Vec<ApplicationPolicyStatement>>,
}

/// <p>Policy statement applied to the application.</p>
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct ApplicationPolicyStatement {
    /// <p>A list of supported actions:\n\n GetApplication \n \n\n CreateCloudFormationChangeSet \n \n\n ListApplicationVersions \n \n\n SearchApplications \n \n\n Deploy (Note: This action enables all other actions above.)</p>
    #[serde(rename = "Actions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<Vec<String>>,
    /// <p>An AWS account ID, or * to make the application public.</p>
    #[serde(rename = "Principals")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub principals: Option<Vec<String>>,
    /// <p>A unique ID for the statement.</p>
    #[serde(rename = "StatementId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_id: Option<String>,
}

/// <p>Summary of details about the application.</p>
#[derive(Default, Debug, Clone, Deserialize)]
pub struct ApplicationSummary {
    /// <p>The application ARN.</p>
    #[serde(rename = "ApplicationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    /// <p>The name of the author publishing the app\nMin Length=1. Max Length=127.\nPattern &quot;^<a href="(%5Ba-z0-9%5D%7C-(?!-))*%5Ba-z0-9%5D">a-z0-9</a>?$&quot;;</p>
    #[serde(rename = "Author")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
    /// <p>The date/time this resource was created.</p>
    #[serde(rename = "CreationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<String>,
    /// <p>The description of the application.\nMin Length=1. Max Length=256</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>Labels to improve discovery of apps in search results.\nMin Length=1. Max Length=127. Maximum number of labels: 10\nPattern: &quot;^[a-zA-Z0-9+\-_:\/@]+$&quot;;</p>
    #[serde(rename = "Labels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<String>>,
    /// <p>The name of the application.\nMin Length=1. Max Length=140\nPattern: &quot;[a-zA-Z0-9\-]+&quot;;</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>A valid identifier from https://spdx.org/licenses/ .</p>
    #[serde(rename = "SpdxLicenseId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spdx_license_id: Option<String>,
}

/// <p>List of version summaries for the application.</p>
#[derive(Default, Debug, Clone)]
pub struct ApplicationVersionPage {
    /// <p>The token to request the next page of results.</p>
    pub next_token: Option<String>,
    /// <p>Array of version summaries for the application.</p>
    pub versions: Option<Vec<VersionSummary>>,
}

/// <p>Details of the change set.</p>
#[derive(Default, Debug, Clone)]
pub struct ChangeSetDetails {
    /// <p>The application Amazon Resource Name (ARN).</p>
    pub application_id: Option<String>,
    /// <p>The ARN of the change set.\nLength Constraints: Minimum length of 1.\nPattern: arn:[-a-zA-Z0-9:/]*</p>
    pub change_set_id: Option<String>,
    /// <p>The semantic version of the application:\n\n https://semver.org/</p>
    pub semantic_version: Option<String>,
    /// <p>The unique ID of the stack.</p>
    pub stack_id: Option<String>,
}

/// <p>Create application request.</p>
#[derive(Default, Debug, Clone)]
pub struct CreateApplicationInput {
    /// <p>The name of the author publishing the app.\nMin Length=1. Max Length=127.\nPattern &quot;^<a href="(%5Ba-z0-9%5D%7C-(?!-))*%5Ba-z0-9%5D">a-z0-9</a>?$&quot;;</p>
    pub author: Option<String>,
    /// <p>The description of the application.\nMin Length=1. Max Length=256</p>
    pub description: Option<String>,
    /// <p>Labels to improve discovery of apps in search results.\nMin Length=1. Max Length=127. Maximum number of labels: 10\nPattern: &quot;^[a-zA-Z0-9+\-_:\/@]+$&quot;;</p>
    pub labels: Option<Vec<String>>,
    /// <p>A raw text file that contains the license of the app that matches the spdxLicenseID of your application.\nMax size 5 MB</p>
    pub license_body: Option<String>,
    /// <p>A link to a license file of the app that matches the spdxLicenseID of your application.\nMax size 5 MB</p>
    pub license_url: Option<String>,
    /// <p>The name of the application you want to publish.\nMin Length=1. Max Length=140\nPattern: &quot;[a-zA-Z0-9\-]+&quot;;</p>
    pub name: Option<String>,
    /// <p>A raw text Readme file that contains a more detailed description of the application and how it works in markdown language.\nMax size 5 MB</p>
    pub readme_body: Option<String>,
    /// <p>A link to the Readme file that contains a more detailed description of the application and how it works in markdown language.\nMax size 5 MB</p>
    pub readme_url: Option<String>,
    /// <p>The semantic version of the application:\n\n https://semver.org/</p>
    pub semantic_version: Option<String>,
    /// <p>A link to a public repository for the source code of your application.</p>
    pub source_code_url: Option<String>,
    /// <p>A valid identifier from https://spdx.org/licenses/ .</p>
    pub spdx_license_id: Option<String>,
    /// <p>The raw packaged SAM template of your application.</p>
    pub template_body: Option<String>,
    /// <p>A link to the packaged SAM template of your application.</p>
    pub template_url: Option<String>,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct CreateApplicationRequest {
    /// <p>The name of the author publishing the app.\nMin Length=1. Max Length=127.\nPattern &quot;^<a href="(%5Ba-z0-9%5D%7C-(?!-))*%5Ba-z0-9%5D">a-z0-9</a>?$&quot;;</p>
    #[serde(rename = "Author")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
    /// <p>The description of the application.\nMin Length=1. Max Length=256</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>Labels to improve discovery of apps in search results.\nMin Length=1. Max Length=127. Maximum number of labels: 10\nPattern: &quot;^[a-zA-Z0-9+\-_:\/@]+$&quot;;</p>
    #[serde(rename = "Labels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<String>>,
    /// <p>A raw text file that contains the license of the app that matches the spdxLicenseID of your application.\nMax size 5 MB</p>
    #[serde(rename = "LicenseBody")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_body: Option<String>,
    /// <p>A link to a license file of the app that matches the spdxLicenseID of your application.\nMax size 5 MB</p>
    #[serde(rename = "LicenseUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_url: Option<String>,
    /// <p>The name of the application you want to publish.\nMin Length=1. Max Length=140\nPattern: &quot;[a-zA-Z0-9\-]+&quot;;</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>A raw text Readme file that contains a more detailed description of the application and how it works in markdown language.\nMax size 5 MB</p>
    #[serde(rename = "ReadmeBody")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub readme_body: Option<String>,
    /// <p>A link to the Readme file that contains a more detailed description of the application and how it works in markdown language.\nMax size 5 MB</p>
    #[serde(rename = "ReadmeUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub readme_url: Option<String>,
    /// <p>The semantic version of the application:\n\n https://semver.org/</p>
    #[serde(rename = "SemanticVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub semantic_version: Option<String>,
    /// <p>A link to a public repository for the source code of your application.</p>
    #[serde(rename = "SourceCodeUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_code_url: Option<String>,
    /// <p>A valid identifier from https://spdx.org/licenses/ .</p>
    #[serde(rename = "SpdxLicenseId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spdx_license_id: Option<String>,
    /// <p>The raw packaged SAM template of your application.</p>
    #[serde(rename = "TemplateBody")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_body: Option<String>,
    /// <p>A link to the packaged SAM template of your application.</p>
    #[serde(rename = "TemplateUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_url: Option<String>,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct CreateApplicationResponse {
    /// <p>The application Amazon Resource Name (ARN).</p>
    #[serde(rename = "ApplicationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    /// <p>The name of the author publishing the app.\nMin Length=1. Max Length=127.\nPattern &quot;^<a href="(%5Ba-z0-9%5D%7C-(?!-))*%5Ba-z0-9%5D">a-z0-9</a>?$&quot;;</p>
    #[serde(rename = "Author")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
    /// <p>The date/time this resource was created.</p>
    #[serde(rename = "CreationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<String>,
    /// <p>The description of the application.\nMin Length=1. Max Length=256</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>Labels to improve discovery of apps in search results.\nMin Length=1. Max Length=127. Maximum number of labels: 10\nPattern: &quot;^[a-zA-Z0-9+\-_:\/@]+$&quot;;</p>
    #[serde(rename = "Labels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<String>>,
    /// <p>A link to a license file of the app that matches the spdxLicenseID of your application.\nMax size 5 MB</p>
    #[serde(rename = "LicenseUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_url: Option<String>,
    /// <p>The name of the application.\nMin Length=1. Max Length=140\nPattern: &quot;[a-zA-Z0-9\-]+&quot;;</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>A link to the Readme file that contains a more detailed description of the application and how it works in markdown language.\nMax size 5 MB</p>
    #[serde(rename = "ReadmeUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub readme_url: Option<String>,
    /// <p>A valid identifier from https://spdx.org/licenses/.</p>
    #[serde(rename = "SpdxLicenseId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spdx_license_id: Option<String>,
    /// <p>Version information about the application.</p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<Version>,
}

/// <p>Create version request.</p>
#[derive(Default, Debug, Clone)]
pub struct CreateApplicationVersionInput {
    /// <p>A link to a public repository for the source code of your application.</p>
    pub source_code_url: Option<String>,
    /// <p>The raw packaged SAM template of your application.</p>
    pub template_body: Option<String>,
    /// <p>A link to the packaged SAM template of your application.</p>
    pub template_url: Option<String>,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct CreateApplicationVersionRequest {
    /// <p>The id of the application to create a new version for</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    /// <p>The semantic version of the new version</p>
    #[serde(rename = "SemanticVersion")]
    pub semantic_version: String,
    /// <p>A link to a public repository for the source code of your application.</p>
    #[serde(rename = "SourceCodeUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_code_url: Option<String>,
    /// <p>The raw packaged SAM template of your application.</p>
    #[serde(rename = "TemplateBody")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_body: Option<String>,
    /// <p>A link to the packaged SAM template of your application.</p>
    #[serde(rename = "TemplateUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_url: Option<String>,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct CreateApplicationVersionResponse {
    /// <p>The application Amazon Resource Name (ARN).</p>
    #[serde(rename = "ApplicationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    /// <p>The date/time this resource was created.</p>
    #[serde(rename = "CreationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<String>,
    /// <p>Array of parameter types supported by the application.</p>
    #[serde(rename = "ParameterDefinitions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_definitions: Option<Vec<ParameterDefinition>>,
    /// <p>The semantic version of the application:\n\n https://semver.org/</p>
    #[serde(rename = "SemanticVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub semantic_version: Option<String>,
    /// <p>A link to a public repository for the source code of your application.</p>
    #[serde(rename = "SourceCodeUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_code_url: Option<String>,
    /// <p>A link to the packaged SAM template of your application.</p>
    #[serde(rename = "TemplateUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_url: Option<String>,
}

/// <p>Create application ChangeSet request.</p>
#[derive(Default, Debug, Clone)]
pub struct CreateCloudFormationChangeSetInput {
    /// <p>A list of parameter values for the parameters of the application.</p>
    pub parameter_overrides: Option<Vec<ParameterValue>>,
    /// <p>The semantic version of the application:\n\n https://semver.org/</p>
    pub semantic_version: Option<String>,
    /// <p>The name or the unique ID of the stack for which you are creating a change set. AWS CloudFormation generates\n the change set by comparing this stack&#39;s information with the information that you submit, such as a modified\n template or different parameter input values. \nConstraints: Minimum length of 1.\nPattern: ([a-zA-Z][-a-zA-Z0-9]<em>)|(arn:\b(aws|aws-us-gov|aws-cn)\b:[-a-zA-Z0-9:/._+]</em>)</p>
    pub stack_name: Option<String>,
}

/// <p>Create application ChangeSet request</p>
#[derive(Default, Debug, Clone, Serialize)]
pub struct CreateCloudFormationChangeSetRequest {
    /// <p>The id of the application to create the ChangeSet for</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    /// <p>A list of parameter values for the parameters of the application.</p>
    #[serde(rename = "ParameterOverrides")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_overrides: Option<Vec<ParameterValue>>,
    /// <p>The semantic version of the application:\n\n https://semver.org/</p>
    #[serde(rename = "SemanticVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub semantic_version: Option<String>,
    /// <p>The name or the unique ID of the stack for which you are creating a change set. AWS CloudFormation generates\n the change set by comparing this stack&#39;s information with the information that you submit, such as a modified\n template or different parameter input values. \nConstraints: Minimum length of 1.\nPattern: ([a-zA-Z][-a-zA-Z0-9]<em>)|(arn:\b(aws|aws-us-gov|aws-cn)\b:[-a-zA-Z0-9:/._+]</em>)</p>
    #[serde(rename = "StackName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_name: Option<String>,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct CreateCloudFormationChangeSetResponse {
    /// <p>The application Amazon Resource Name (ARN).</p>
    #[serde(rename = "ApplicationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    /// <p>The ARN of the change set.\nLength Constraints: Minimum length of 1.\nPattern: arn:[-a-zA-Z0-9:/]*</p>
    #[serde(rename = "ChangeSetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_set_id: Option<String>,
    /// <p>The semantic version of the application:\n\n https://semver.org/</p>
    #[serde(rename = "SemanticVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub semantic_version: Option<String>,
    /// <p>The unique ID of the stack.</p>
    #[serde(rename = "StackId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_id: Option<String>,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct GetApplicationPolicyRequest {
    /// <p>The id of the application to get policy for</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct GetApplicationPolicyResponse {
    /// <p>Array of policy statements applied to the application.</p>
    #[serde(rename = "Statements")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statements: Option<Vec<ApplicationPolicyStatement>>,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct GetApplicationRequest {
    /// <p>The id of the application to get</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    /// <p>The semantic version of the application to get</p>
    #[serde(rename = "SemanticVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub semantic_version: Option<String>,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct GetApplicationResponse {
    /// <p>The application Amazon Resource Name (ARN).</p>
    #[serde(rename = "ApplicationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    /// <p>The name of the author publishing the app.\nMin Length=1. Max Length=127.\nPattern &quot;^<a href="(%5Ba-z0-9%5D%7C-(?!-))*%5Ba-z0-9%5D">a-z0-9</a>?$&quot;;</p>
    #[serde(rename = "Author")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
    /// <p>The date/time this resource was created.</p>
    #[serde(rename = "CreationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<String>,
    /// <p>The description of the application.\nMin Length=1. Max Length=256</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>Labels to improve discovery of apps in search results.\nMin Length=1. Max Length=127. Maximum number of labels: 10\nPattern: &quot;^[a-zA-Z0-9+\-_:\/@]+$&quot;;</p>
    #[serde(rename = "Labels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<String>>,
    /// <p>A link to a license file of the app that matches the spdxLicenseID of your application.\nMax size 5 MB</p>
    #[serde(rename = "LicenseUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_url: Option<String>,
    /// <p>The name of the application.\nMin Length=1. Max Length=140\nPattern: &quot;[a-zA-Z0-9\-]+&quot;;</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>A link to the Readme file that contains a more detailed description of the application and how it works in markdown language.\nMax size 5 MB</p>
    #[serde(rename = "ReadmeUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub readme_url: Option<String>,
    /// <p>A valid identifier from https://spdx.org/licenses/.</p>
    #[serde(rename = "SpdxLicenseId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spdx_license_id: Option<String>,
    /// <p>Version information about the application.</p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<Version>,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct ListApplicationVersionsRequest {
    /// <p>The id of the application to list</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    /// <p>The total number of items to return</p>
    #[serde(rename = "MaxItems")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i64>,
    /// <p>A token to specify where to start paginating</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct ListApplicationVersionsResponse {
    /// <p>The token to request the next page of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Array of version summaries for the application.</p>
    #[serde(rename = "Versions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub versions: Option<Vec<VersionSummary>>,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct ListApplicationsRequest {
    /// <p>The total number of items to return</p>
    #[serde(rename = "MaxItems")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i64>,
    /// <p>A token to specify where to start paginating</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct ListApplicationsResponse {
    /// <p>Array of application summaries.</p>
    #[serde(rename = "Applications")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub applications: Option<Vec<ApplicationSummary>>,
    /// <p>The token to request the next page of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>Parameters supported by the application.</p>
#[derive(Default, Debug, Clone, Deserialize)]
pub struct ParameterDefinition {
    /// <p>A regular expression that represents the patterns to allow for String types.</p>
    #[serde(rename = "AllowedPattern")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_pattern: Option<String>,
    /// <p>Array containing the list of values allowed for the parameter.</p>
    #[serde(rename = "AllowedValues")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_values: Option<Vec<String>>,
    /// <p>A string that explains a constraint when the constraint is violated. For example, without a constraint description,\n a parameter that has an allowed pattern of [A-Za-z0-9]+ displays the following error message when the user\n specifies an invalid value:\n\n Malformed input-Parameter MyParameter must match pattern [A-Za-z0-9]+ \n \nBy adding a constraint description, such as &quot;must contain only uppercase and lowercase letters, and numbers,&quot; you can display\n the following customized error message:\n\n Malformed input-Parameter MyParameter must contain only uppercase and lowercase letters and numbers.</p>
    #[serde(rename = "ConstraintDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constraint_description: Option<String>,
    /// <p>A value of the appropriate type for the template to use if no value is specified when a stack is created.\n If you define constraints for the parameter, you must specify a value that adheres to those constraints.</p>
    #[serde(rename = "DefaultValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_value: Option<String>,
    /// <p>A string of up to 4,000 characters that describes the parameter.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>An integer value that determines the largest number of characters you want to allow for String types.</p>
    #[serde(rename = "MaxLength")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_length: Option<i64>,
    /// <p>A numeric value that determines the largest numeric value you want to allow for Number types.</p>
    #[serde(rename = "MaxValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_value: Option<i64>,
    /// <p>An integer value that determines the smallest number of characters you want to allow for String types.</p>
    #[serde(rename = "MinLength")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_length: Option<i64>,
    /// <p>A numeric value that determines the smallest numeric value you want to allow for Number types.</p>
    #[serde(rename = "MinValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_value: Option<i64>,
    /// <p>The name of the parameter.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>Whether to mask the parameter value whenever anyone makes a call that describes the stack. If you set the\n value to true, the parameter value is masked with asterisks (*****).</p>
    #[serde(rename = "NoEcho")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_echo: Option<bool>,
    /// <p>A list of SAM resources that use this parameter.</p>
    #[serde(rename = "ReferencedByResources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub referenced_by_resources: Option<Vec<String>>,
    /// <p>The type of the parameter.\nValid values: String | Number | List<Number> | CommaDelimitedList \n \n\n String : A literal string.\nFor example, users could specify &quot;MyUserName&quot; .\n\n Number : An integer or float. AWS CloudFormation validates the parameter value as a number; however, when you use the\n parameter elsewhere in your template (for example, by using the Ref intrinsic function), the parameter value becomes a string.\nFor example, users could specify &quot;8888&quot; .\n\n List<Number> : An array of integers or floats that are separated by commas. AWS CloudFormation validates the parameter value as numbers; however, when\n you use the parameter elsewhere in your template (for example, by using the Ref intrinsic function), the parameter value becomes a list of strings.\nFor example, users could specify &quot;80,20&quot;, and a Ref results in [&quot;80&quot;,&quot;20&quot;] .\n\n CommaDelimitedList : An array of literal strings that are separated by commas. The total number of strings should be one more than the total number of commas.\n Also, each member string is space-trimmed.\nFor example, users could specify &quot;test,dev,prod&quot;, and a Ref results in [&quot;test&quot;,&quot;dev&quot;,&quot;prod&quot;] .</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>Parameter value of the application.</p>
#[derive(Default, Debug, Clone, Serialize)]
pub struct ParameterValue {
    /// <p>The key associated with the parameter. If you don&#39;t specify a key and value for a particular parameter, AWS CloudFormation\n uses the default value that is specified in your template.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The input value associated with the parameter.</p>
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// <p>Put policy request</p>
#[derive(Default, Debug, Clone, Serialize)]
pub struct PutApplicationPolicyRequest {
    /// <p>The id of the application to put policy for</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    /// <p>Array of policy statements applied to the application.</p>
    #[serde(rename = "Statements")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statements: Option<Vec<ApplicationPolicyStatement>>,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct PutApplicationPolicyResponse {
    /// <p>Array of policy statements applied to the application.</p>
    #[serde(rename = "Statements")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statements: Option<Vec<ApplicationPolicyStatement>>,
}

/// <p>Update application request.</p>
#[derive(Default, Debug, Clone)]
pub struct UpdateApplicationInput {
    /// <p>The name of the author publishing the app.\nMin Length=1. Max Length=127.\nPattern &quot;^<a href="(%5Ba-z0-9%5D%7C-(?!-))*%5Ba-z0-9%5D">a-z0-9</a>?$&quot;;</p>
    pub author: Option<String>,
    /// <p>The description of the application.\nMin Length=1. Max Length=256</p>
    pub description: Option<String>,
    /// <p>Labels to improve discovery of apps in search results.\nMin Length=1. Max Length=127. Maximum number of labels: 10\nPattern: &quot;^[a-zA-Z0-9+\-_:\/@]+$&quot;;</p>
    pub labels: Option<Vec<String>>,
    /// <p>A raw text Readme file that contains a more detailed description of the application and how it works in markdown language.\nMax size 5 MB</p>
    pub readme_body: Option<String>,
    /// <p>A link to the Readme file that contains a more detailed description of the application and how it works in markdown language.\nMax size 5 MB</p>
    pub readme_url: Option<String>,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct UpdateApplicationRequest {
    /// <p>The id of the application to update</p>
    #[serde(rename = "ApplicationId")]
    pub application_id: String,
    /// <p>The name of the author publishing the app.\nMin Length=1. Max Length=127.\nPattern &quot;^<a href="(%5Ba-z0-9%5D%7C-(?!-))*%5Ba-z0-9%5D">a-z0-9</a>?$&quot;;</p>
    #[serde(rename = "Author")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
    /// <p>The description of the application.\nMin Length=1. Max Length=256</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>Labels to improve discovery of apps in search results.\nMin Length=1. Max Length=127. Maximum number of labels: 10\nPattern: &quot;^[a-zA-Z0-9+\-_:\/@]+$&quot;;</p>
    #[serde(rename = "Labels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<String>>,
    /// <p>A raw text Readme file that contains a more detailed description of the application and how it works in markdown language.\nMax size 5 MB</p>
    #[serde(rename = "ReadmeBody")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub readme_body: Option<String>,
    /// <p>A link to the Readme file that contains a more detailed description of the application and how it works in markdown language.\nMax size 5 MB</p>
    #[serde(rename = "ReadmeUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub readme_url: Option<String>,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct UpdateApplicationResponse {
    /// <p>The application Amazon Resource Name (ARN).</p>
    #[serde(rename = "ApplicationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    /// <p>The name of the author publishing the app.\nMin Length=1. Max Length=127.\nPattern &quot;^<a href="(%5Ba-z0-9%5D%7C-(?!-))*%5Ba-z0-9%5D">a-z0-9</a>?$&quot;;</p>
    #[serde(rename = "Author")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
    /// <p>The date/time this resource was created.</p>
    #[serde(rename = "CreationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<String>,
    /// <p>The description of the application.\nMin Length=1. Max Length=256</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>Labels to improve discovery of apps in search results.\nMin Length=1. Max Length=127. Maximum number of labels: 10\nPattern: &quot;^[a-zA-Z0-9+\-_:\/@]+$&quot;;</p>
    #[serde(rename = "Labels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<String>>,
    /// <p>A link to a license file of the app that matches the spdxLicenseID of your application.\nMax size 5 MB</p>
    #[serde(rename = "LicenseUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_url: Option<String>,
    /// <p>The name of the application.\nMin Length=1. Max Length=140\nPattern: &quot;[a-zA-Z0-9\-]+&quot;;</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>A link to the Readme file that contains a more detailed description of the application and how it works in markdown language.\nMax size 5 MB</p>
    #[serde(rename = "ReadmeUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub readme_url: Option<String>,
    /// <p>A valid identifier from https://spdx.org/licenses/.</p>
    #[serde(rename = "SpdxLicenseId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spdx_license_id: Option<String>,
    /// <p>Version information about the application.</p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<Version>,
}

/// <p>Application version details.</p>
#[derive(Default, Debug, Clone, Deserialize)]
pub struct Version {
    /// <p>The application Amazon Resource Name (ARN).</p>
    #[serde(rename = "ApplicationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    /// <p>The date/time this resource was created.</p>
    #[serde(rename = "CreationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<String>,
    /// <p>Array of parameter types supported by the application.</p>
    #[serde(rename = "ParameterDefinitions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_definitions: Option<Vec<ParameterDefinition>>,
    /// <p>The semantic version of the application:\n\n https://semver.org/</p>
    #[serde(rename = "SemanticVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub semantic_version: Option<String>,
    /// <p>A link to a public repository for the source code of your application.</p>
    #[serde(rename = "SourceCodeUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_code_url: Option<String>,
    /// <p>A link to the packaged SAM template of your application.</p>
    #[serde(rename = "TemplateUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_url: Option<String>,
}

/// <p>Application version summary.</p>
#[derive(Default, Debug, Clone, Deserialize)]
pub struct VersionSummary {
    /// <p>The application Amazon Resource Name (ARN).</p>
    #[serde(rename = "ApplicationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    /// <p>The date/time this resource was created.</p>
    #[serde(rename = "CreationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<String>,
    /// <p>The semantic version of the application:\n\n https://semver.org/</p>
    #[serde(rename = "SemanticVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub semantic_version: Option<String>,
    /// <p>A link to a public repository for the source code of your application.</p>
    #[serde(rename = "SourceCodeUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_code_url: Option<String>,
}

/// Errors returned by CreateApplication
#[derive(Debug, PartialEq)]
pub enum CreateApplicationError {
    /// <p>One of the parameters in the request is invalid.</p>
    BadRequest(String),
    /// <p>The resource already exists.</p>
    Conflict(String),
    /// <p>The client is not authenticated.</p>
    Forbidden(String),
    /// <p>The AWS Serverless Application Repository service encountered an internal error.</p>
    InternalServerError(String),
    /// <p>The client is sending more than the allowed number of requests per unit time.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreateApplicationError {
    pub fn from_body(body: &str) -> CreateApplicationError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "BadRequestException" => {
                        CreateApplicationError::BadRequest(String::from(error_message))
                    }
                    "ConflictException" => {
                        CreateApplicationError::Conflict(String::from(error_message))
                    }
                    "ForbiddenException" => {
                        CreateApplicationError::Forbidden(String::from(error_message))
                    }
                    "InternalServerErrorException" => {
                        CreateApplicationError::InternalServerError(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        CreateApplicationError::TooManyRequests(String::from(error_message))
                    }
                    "ValidationException" => {
                        CreateApplicationError::Validation(error_message.to_string())
                    }
                    _ => CreateApplicationError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateApplicationError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateApplicationError {
    fn from(err: serde_json::error::Error) -> CreateApplicationError {
        CreateApplicationError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateApplicationError {
    fn from(err: CredentialsError) -> CreateApplicationError {
        CreateApplicationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateApplicationError {
    fn from(err: HttpDispatchError) -> CreateApplicationError {
        CreateApplicationError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateApplicationError {
    fn from(err: io::Error) -> CreateApplicationError {
        CreateApplicationError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateApplicationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateApplicationError {
    fn description(&self) -> &str {
        match *self {
            CreateApplicationError::BadRequest(ref cause) => cause,
            CreateApplicationError::Conflict(ref cause) => cause,
            CreateApplicationError::Forbidden(ref cause) => cause,
            CreateApplicationError::InternalServerError(ref cause) => cause,
            CreateApplicationError::TooManyRequests(ref cause) => cause,
            CreateApplicationError::Validation(ref cause) => cause,
            CreateApplicationError::Credentials(ref err) => err.description(),
            CreateApplicationError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateApplicationError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateApplicationVersion
#[derive(Debug, PartialEq)]
pub enum CreateApplicationVersionError {
    /// <p>One of the parameters in the request is invalid.</p>
    BadRequest(String),
    /// <p>The resource already exists.</p>
    Conflict(String),
    /// <p>The client is not authenticated.</p>
    Forbidden(String),
    /// <p>The AWS Serverless Application Repository service encountered an internal error.</p>
    InternalServerError(String),
    /// <p>The client is sending more than the allowed number of requests per unit time.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreateApplicationVersionError {
    pub fn from_body(body: &str) -> CreateApplicationVersionError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "BadRequestException" => {
                        CreateApplicationVersionError::BadRequest(String::from(error_message))
                    }
                    "ConflictException" => {
                        CreateApplicationVersionError::Conflict(String::from(error_message))
                    }
                    "ForbiddenException" => {
                        CreateApplicationVersionError::Forbidden(String::from(error_message))
                    }
                    "InternalServerErrorException" => {
                        CreateApplicationVersionError::InternalServerError(String::from(
                            error_message,
                        ))
                    }
                    "TooManyRequestsException" => {
                        CreateApplicationVersionError::TooManyRequests(String::from(error_message))
                    }
                    "ValidationException" => {
                        CreateApplicationVersionError::Validation(error_message.to_string())
                    }
                    _ => CreateApplicationVersionError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateApplicationVersionError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateApplicationVersionError {
    fn from(err: serde_json::error::Error) -> CreateApplicationVersionError {
        CreateApplicationVersionError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateApplicationVersionError {
    fn from(err: CredentialsError) -> CreateApplicationVersionError {
        CreateApplicationVersionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateApplicationVersionError {
    fn from(err: HttpDispatchError) -> CreateApplicationVersionError {
        CreateApplicationVersionError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateApplicationVersionError {
    fn from(err: io::Error) -> CreateApplicationVersionError {
        CreateApplicationVersionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateApplicationVersionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateApplicationVersionError {
    fn description(&self) -> &str {
        match *self {
            CreateApplicationVersionError::BadRequest(ref cause) => cause,
            CreateApplicationVersionError::Conflict(ref cause) => cause,
            CreateApplicationVersionError::Forbidden(ref cause) => cause,
            CreateApplicationVersionError::InternalServerError(ref cause) => cause,
            CreateApplicationVersionError::TooManyRequests(ref cause) => cause,
            CreateApplicationVersionError::Validation(ref cause) => cause,
            CreateApplicationVersionError::Credentials(ref err) => err.description(),
            CreateApplicationVersionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateApplicationVersionError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateCloudFormationChangeSet
#[derive(Debug, PartialEq)]
pub enum CreateCloudFormationChangeSetError {
    /// <p>One of the parameters in the request is invalid.</p>
    BadRequest(String),
    /// <p>The client is not authenticated.</p>
    Forbidden(String),
    /// <p>The AWS Serverless Application Repository service encountered an internal error.</p>
    InternalServerError(String),
    /// <p>The client is sending more than the allowed number of requests per unit time.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreateCloudFormationChangeSetError {
    pub fn from_body(body: &str) -> CreateCloudFormationChangeSetError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "BadRequestException" => {
                        CreateCloudFormationChangeSetError::BadRequest(String::from(error_message))
                    }
                    "ForbiddenException" => {
                        CreateCloudFormationChangeSetError::Forbidden(String::from(error_message))
                    }
                    "InternalServerErrorException" => {
                        CreateCloudFormationChangeSetError::InternalServerError(String::from(
                            error_message,
                        ))
                    }
                    "TooManyRequestsException" => {
                        CreateCloudFormationChangeSetError::TooManyRequests(String::from(
                            error_message,
                        ))
                    }
                    "ValidationException" => {
                        CreateCloudFormationChangeSetError::Validation(error_message.to_string())
                    }
                    _ => CreateCloudFormationChangeSetError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateCloudFormationChangeSetError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateCloudFormationChangeSetError {
    fn from(err: serde_json::error::Error) -> CreateCloudFormationChangeSetError {
        CreateCloudFormationChangeSetError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateCloudFormationChangeSetError {
    fn from(err: CredentialsError) -> CreateCloudFormationChangeSetError {
        CreateCloudFormationChangeSetError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateCloudFormationChangeSetError {
    fn from(err: HttpDispatchError) -> CreateCloudFormationChangeSetError {
        CreateCloudFormationChangeSetError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateCloudFormationChangeSetError {
    fn from(err: io::Error) -> CreateCloudFormationChangeSetError {
        CreateCloudFormationChangeSetError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateCloudFormationChangeSetError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateCloudFormationChangeSetError {
    fn description(&self) -> &str {
        match *self {
            CreateCloudFormationChangeSetError::BadRequest(ref cause) => cause,
            CreateCloudFormationChangeSetError::Forbidden(ref cause) => cause,
            CreateCloudFormationChangeSetError::InternalServerError(ref cause) => cause,
            CreateCloudFormationChangeSetError::TooManyRequests(ref cause) => cause,
            CreateCloudFormationChangeSetError::Validation(ref cause) => cause,
            CreateCloudFormationChangeSetError::Credentials(ref err) => err.description(),
            CreateCloudFormationChangeSetError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateCloudFormationChangeSetError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetApplication
#[derive(Debug, PartialEq)]
pub enum GetApplicationError {
    /// <p>One of the parameters in the request is invalid.</p>
    BadRequest(String),
    /// <p>The client is not authenticated.</p>
    Forbidden(String),
    /// <p>The AWS Serverless Application Repository service encountered an internal error.</p>
    InternalServerError(String),
    /// <p>The resource (for example, an access policy statement) specified in the request does not exist.</p>
    NotFound(String),
    /// <p>The client is sending more than the allowed number of requests per unit time.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetApplicationError {
    pub fn from_body(body: &str) -> GetApplicationError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "BadRequestException" => {
                        GetApplicationError::BadRequest(String::from(error_message))
                    }
                    "ForbiddenException" => {
                        GetApplicationError::Forbidden(String::from(error_message))
                    }
                    "InternalServerErrorException" => {
                        GetApplicationError::InternalServerError(String::from(error_message))
                    }
                    "NotFoundException" => {
                        GetApplicationError::NotFound(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        GetApplicationError::TooManyRequests(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetApplicationError::Validation(error_message.to_string())
                    }
                    _ => GetApplicationError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetApplicationError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetApplicationError {
    fn from(err: serde_json::error::Error) -> GetApplicationError {
        GetApplicationError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetApplicationError {
    fn from(err: CredentialsError) -> GetApplicationError {
        GetApplicationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetApplicationError {
    fn from(err: HttpDispatchError) -> GetApplicationError {
        GetApplicationError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetApplicationError {
    fn from(err: io::Error) -> GetApplicationError {
        GetApplicationError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetApplicationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetApplicationError {
    fn description(&self) -> &str {
        match *self {
            GetApplicationError::BadRequest(ref cause) => cause,
            GetApplicationError::Forbidden(ref cause) => cause,
            GetApplicationError::InternalServerError(ref cause) => cause,
            GetApplicationError::NotFound(ref cause) => cause,
            GetApplicationError::TooManyRequests(ref cause) => cause,
            GetApplicationError::Validation(ref cause) => cause,
            GetApplicationError::Credentials(ref err) => err.description(),
            GetApplicationError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetApplicationError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetApplicationPolicy
#[derive(Debug, PartialEq)]
pub enum GetApplicationPolicyError {
    /// <p>One of the parameters in the request is invalid.</p>
    BadRequest(String),
    /// <p>The client is not authenticated.</p>
    Forbidden(String),
    /// <p>The AWS Serverless Application Repository service encountered an internal error.</p>
    InternalServerError(String),
    /// <p>The resource (for example, an access policy statement) specified in the request does not exist.</p>
    NotFound(String),
    /// <p>The client is sending more than the allowed number of requests per unit time.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetApplicationPolicyError {
    pub fn from_body(body: &str) -> GetApplicationPolicyError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "BadRequestException" => {
                        GetApplicationPolicyError::BadRequest(String::from(error_message))
                    }
                    "ForbiddenException" => {
                        GetApplicationPolicyError::Forbidden(String::from(error_message))
                    }
                    "InternalServerErrorException" => {
                        GetApplicationPolicyError::InternalServerError(String::from(error_message))
                    }
                    "NotFoundException" => {
                        GetApplicationPolicyError::NotFound(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        GetApplicationPolicyError::TooManyRequests(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetApplicationPolicyError::Validation(error_message.to_string())
                    }
                    _ => GetApplicationPolicyError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetApplicationPolicyError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetApplicationPolicyError {
    fn from(err: serde_json::error::Error) -> GetApplicationPolicyError {
        GetApplicationPolicyError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetApplicationPolicyError {
    fn from(err: CredentialsError) -> GetApplicationPolicyError {
        GetApplicationPolicyError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetApplicationPolicyError {
    fn from(err: HttpDispatchError) -> GetApplicationPolicyError {
        GetApplicationPolicyError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetApplicationPolicyError {
    fn from(err: io::Error) -> GetApplicationPolicyError {
        GetApplicationPolicyError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetApplicationPolicyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetApplicationPolicyError {
    fn description(&self) -> &str {
        match *self {
            GetApplicationPolicyError::BadRequest(ref cause) => cause,
            GetApplicationPolicyError::Forbidden(ref cause) => cause,
            GetApplicationPolicyError::InternalServerError(ref cause) => cause,
            GetApplicationPolicyError::NotFound(ref cause) => cause,
            GetApplicationPolicyError::TooManyRequests(ref cause) => cause,
            GetApplicationPolicyError::Validation(ref cause) => cause,
            GetApplicationPolicyError::Credentials(ref err) => err.description(),
            GetApplicationPolicyError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetApplicationPolicyError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListApplicationVersions
#[derive(Debug, PartialEq)]
pub enum ListApplicationVersionsError {
    /// <p>One of the parameters in the request is invalid.</p>
    BadRequest(String),
    /// <p>The client is not authenticated.</p>
    Forbidden(String),
    /// <p>The AWS Serverless Application Repository service encountered an internal error.</p>
    InternalServerError(String),
    /// <p>The resource (for example, an access policy statement) specified in the request does not exist.</p>
    NotFound(String),
    /// <p>The client is sending more than the allowed number of requests per unit time.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListApplicationVersionsError {
    pub fn from_body(body: &str) -> ListApplicationVersionsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "BadRequestException" => {
                        ListApplicationVersionsError::BadRequest(String::from(error_message))
                    }
                    "ForbiddenException" => {
                        ListApplicationVersionsError::Forbidden(String::from(error_message))
                    }
                    "InternalServerErrorException" => {
                        ListApplicationVersionsError::InternalServerError(String::from(
                            error_message,
                        ))
                    }
                    "NotFoundException" => {
                        ListApplicationVersionsError::NotFound(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        ListApplicationVersionsError::TooManyRequests(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListApplicationVersionsError::Validation(error_message.to_string())
                    }
                    _ => ListApplicationVersionsError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListApplicationVersionsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListApplicationVersionsError {
    fn from(err: serde_json::error::Error) -> ListApplicationVersionsError {
        ListApplicationVersionsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListApplicationVersionsError {
    fn from(err: CredentialsError) -> ListApplicationVersionsError {
        ListApplicationVersionsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListApplicationVersionsError {
    fn from(err: HttpDispatchError) -> ListApplicationVersionsError {
        ListApplicationVersionsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListApplicationVersionsError {
    fn from(err: io::Error) -> ListApplicationVersionsError {
        ListApplicationVersionsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListApplicationVersionsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListApplicationVersionsError {
    fn description(&self) -> &str {
        match *self {
            ListApplicationVersionsError::BadRequest(ref cause) => cause,
            ListApplicationVersionsError::Forbidden(ref cause) => cause,
            ListApplicationVersionsError::InternalServerError(ref cause) => cause,
            ListApplicationVersionsError::NotFound(ref cause) => cause,
            ListApplicationVersionsError::TooManyRequests(ref cause) => cause,
            ListApplicationVersionsError::Validation(ref cause) => cause,
            ListApplicationVersionsError::Credentials(ref err) => err.description(),
            ListApplicationVersionsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListApplicationVersionsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListApplications
#[derive(Debug, PartialEq)]
pub enum ListApplicationsError {
    /// <p>One of the parameters in the request is invalid.</p>
    BadRequest(String),
    /// <p>The client is not authenticated.</p>
    Forbidden(String),
    /// <p>The AWS Serverless Application Repository service encountered an internal error.</p>
    InternalServerError(String),
    /// <p>The resource (for example, an access policy statement) specified in the request does not exist.</p>
    NotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListApplicationsError {
    pub fn from_body(body: &str) -> ListApplicationsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "BadRequestException" => {
                        ListApplicationsError::BadRequest(String::from(error_message))
                    }
                    "ForbiddenException" => {
                        ListApplicationsError::Forbidden(String::from(error_message))
                    }
                    "InternalServerErrorException" => {
                        ListApplicationsError::InternalServerError(String::from(error_message))
                    }
                    "NotFoundException" => {
                        ListApplicationsError::NotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListApplicationsError::Validation(error_message.to_string())
                    }
                    _ => ListApplicationsError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListApplicationsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListApplicationsError {
    fn from(err: serde_json::error::Error) -> ListApplicationsError {
        ListApplicationsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListApplicationsError {
    fn from(err: CredentialsError) -> ListApplicationsError {
        ListApplicationsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListApplicationsError {
    fn from(err: HttpDispatchError) -> ListApplicationsError {
        ListApplicationsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListApplicationsError {
    fn from(err: io::Error) -> ListApplicationsError {
        ListApplicationsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListApplicationsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListApplicationsError {
    fn description(&self) -> &str {
        match *self {
            ListApplicationsError::BadRequest(ref cause) => cause,
            ListApplicationsError::Forbidden(ref cause) => cause,
            ListApplicationsError::InternalServerError(ref cause) => cause,
            ListApplicationsError::NotFound(ref cause) => cause,
            ListApplicationsError::Validation(ref cause) => cause,
            ListApplicationsError::Credentials(ref err) => err.description(),
            ListApplicationsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListApplicationsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by PutApplicationPolicy
#[derive(Debug, PartialEq)]
pub enum PutApplicationPolicyError {
    /// <p>One of the parameters in the request is invalid.</p>
    BadRequest(String),
    /// <p>The client is not authenticated.</p>
    Forbidden(String),
    /// <p>The AWS Serverless Application Repository service encountered an internal error.</p>
    InternalServerError(String),
    /// <p>The resource (for example, an access policy statement) specified in the request does not exist.</p>
    NotFound(String),
    /// <p>The client is sending more than the allowed number of requests per unit time.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl PutApplicationPolicyError {
    pub fn from_body(body: &str) -> PutApplicationPolicyError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "BadRequestException" => {
                        PutApplicationPolicyError::BadRequest(String::from(error_message))
                    }
                    "ForbiddenException" => {
                        PutApplicationPolicyError::Forbidden(String::from(error_message))
                    }
                    "InternalServerErrorException" => {
                        PutApplicationPolicyError::InternalServerError(String::from(error_message))
                    }
                    "NotFoundException" => {
                        PutApplicationPolicyError::NotFound(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        PutApplicationPolicyError::TooManyRequests(String::from(error_message))
                    }
                    "ValidationException" => {
                        PutApplicationPolicyError::Validation(error_message.to_string())
                    }
                    _ => PutApplicationPolicyError::Unknown(String::from(body)),
                }
            }
            Err(_) => PutApplicationPolicyError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for PutApplicationPolicyError {
    fn from(err: serde_json::error::Error) -> PutApplicationPolicyError {
        PutApplicationPolicyError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for PutApplicationPolicyError {
    fn from(err: CredentialsError) -> PutApplicationPolicyError {
        PutApplicationPolicyError::Credentials(err)
    }
}
impl From<HttpDispatchError> for PutApplicationPolicyError {
    fn from(err: HttpDispatchError) -> PutApplicationPolicyError {
        PutApplicationPolicyError::HttpDispatch(err)
    }
}
impl From<io::Error> for PutApplicationPolicyError {
    fn from(err: io::Error) -> PutApplicationPolicyError {
        PutApplicationPolicyError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for PutApplicationPolicyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutApplicationPolicyError {
    fn description(&self) -> &str {
        match *self {
            PutApplicationPolicyError::BadRequest(ref cause) => cause,
            PutApplicationPolicyError::Forbidden(ref cause) => cause,
            PutApplicationPolicyError::InternalServerError(ref cause) => cause,
            PutApplicationPolicyError::NotFound(ref cause) => cause,
            PutApplicationPolicyError::TooManyRequests(ref cause) => cause,
            PutApplicationPolicyError::Validation(ref cause) => cause,
            PutApplicationPolicyError::Credentials(ref err) => err.description(),
            PutApplicationPolicyError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            PutApplicationPolicyError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateApplication
#[derive(Debug, PartialEq)]
pub enum UpdateApplicationError {
    /// <p>One of the parameters in the request is invalid.</p>
    BadRequest(String),
    /// <p>The resource already exists.</p>
    Conflict(String),
    /// <p>The client is not authenticated.</p>
    Forbidden(String),
    /// <p>The AWS Serverless Application Repository service encountered an internal error.</p>
    InternalServerError(String),
    /// <p>The resource (for example, an access policy statement) specified in the request does not exist.</p>
    NotFound(String),
    /// <p>The client is sending more than the allowed number of requests per unit time.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl UpdateApplicationError {
    pub fn from_body(body: &str) -> UpdateApplicationError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "BadRequestException" => {
                        UpdateApplicationError::BadRequest(String::from(error_message))
                    }
                    "ConflictException" => {
                        UpdateApplicationError::Conflict(String::from(error_message))
                    }
                    "ForbiddenException" => {
                        UpdateApplicationError::Forbidden(String::from(error_message))
                    }
                    "InternalServerErrorException" => {
                        UpdateApplicationError::InternalServerError(String::from(error_message))
                    }
                    "NotFoundException" => {
                        UpdateApplicationError::NotFound(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        UpdateApplicationError::TooManyRequests(String::from(error_message))
                    }
                    "ValidationException" => {
                        UpdateApplicationError::Validation(error_message.to_string())
                    }
                    _ => UpdateApplicationError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateApplicationError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateApplicationError {
    fn from(err: serde_json::error::Error) -> UpdateApplicationError {
        UpdateApplicationError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateApplicationError {
    fn from(err: CredentialsError) -> UpdateApplicationError {
        UpdateApplicationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateApplicationError {
    fn from(err: HttpDispatchError) -> UpdateApplicationError {
        UpdateApplicationError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateApplicationError {
    fn from(err: io::Error) -> UpdateApplicationError {
        UpdateApplicationError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateApplicationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateApplicationError {
    fn description(&self) -> &str {
        match *self {
            UpdateApplicationError::BadRequest(ref cause) => cause,
            UpdateApplicationError::Conflict(ref cause) => cause,
            UpdateApplicationError::Forbidden(ref cause) => cause,
            UpdateApplicationError::InternalServerError(ref cause) => cause,
            UpdateApplicationError::NotFound(ref cause) => cause,
            UpdateApplicationError::TooManyRequests(ref cause) => cause,
            UpdateApplicationError::Validation(ref cause) => cause,
            UpdateApplicationError::Credentials(ref err) => err.description(),
            UpdateApplicationError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpdateApplicationError::Unknown(ref cause) => cause,
        }
    }
}
/// Trait representing the capabilities of the AWSServerlessApplicationRepository API. AWSServerlessApplicationRepository clients implement this trait.
pub trait ServerlessRepo {
    /// <p>Creates an application, optionally including an AWS SAM file to create the first application version in the same call.</p>
    fn create_application(
        &self,
        input: &CreateApplicationRequest,
    ) -> RusotoFuture<CreateApplicationResponse, CreateApplicationError>;

    /// <p>Creates an application version.</p>
    fn create_application_version(
        &self,
        input: &CreateApplicationVersionRequest,
    ) -> RusotoFuture<CreateApplicationVersionResponse, CreateApplicationVersionError>;

    /// <p>Creates an AWS CloudFormation ChangeSet for the given application.</p>
    fn create_cloud_formation_change_set(
        &self,
        input: &CreateCloudFormationChangeSetRequest,
    ) -> RusotoFuture<CreateCloudFormationChangeSetResponse, CreateCloudFormationChangeSetError>;

    /// <p>Gets the specified application.</p>
    fn get_application(
        &self,
        input: &GetApplicationRequest,
    ) -> RusotoFuture<GetApplicationResponse, GetApplicationError>;

    /// <p>Gets the policy for the specified application.</p>
    fn get_application_policy(
        &self,
        input: &GetApplicationPolicyRequest,
    ) -> RusotoFuture<GetApplicationPolicyResponse, GetApplicationPolicyError>;

    /// <p>Lists versions for the specified application.</p>
    fn list_application_versions(
        &self,
        input: &ListApplicationVersionsRequest,
    ) -> RusotoFuture<ListApplicationVersionsResponse, ListApplicationVersionsError>;

    /// <p>Lists applications owned by the requester.</p>
    fn list_applications(
        &self,
        input: &ListApplicationsRequest,
    ) -> RusotoFuture<ListApplicationsResponse, ListApplicationsError>;

    /// <p>Puts the policy for the specified application.</p>
    fn put_application_policy(
        &self,
        input: &PutApplicationPolicyRequest,
    ) -> RusotoFuture<PutApplicationPolicyResponse, PutApplicationPolicyError>;

    /// <p>Updates the specified application.</p>
    fn update_application(
        &self,
        input: &UpdateApplicationRequest,
    ) -> RusotoFuture<UpdateApplicationResponse, UpdateApplicationError>;
}
/// A client for the AWSServerlessApplicationRepository API.
pub struct ServerlessRepoClient<P = CredentialsProvider, D = RequestDispatcher>
where
    P: ProvideAwsCredentials,
    D: DispatchSignedRequest,
{
    inner: ClientInner<P, D>,
    region: region::Region,
}

impl ServerlessRepoClient {
    /// Creates a simple client backed by an implicit event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    ///
    /// See the `rusoto_core::reactor` module for more details.
    pub fn simple(region: region::Region) -> ServerlessRepoClient {
        ServerlessRepoClient::new(
            RequestDispatcher::default(),
            CredentialsProvider::default(),
            region,
        )
    }
}

impl<P, D> ServerlessRepoClient<P, D>
where
    P: ProvideAwsCredentials,
    D: DispatchSignedRequest,
{
    pub fn new(request_dispatcher: D, credentials_provider: P, region: region::Region) -> Self {
        ServerlessRepoClient {
            inner: ClientInner::new(credentials_provider, request_dispatcher),
            region: region,
        }
    }
}

impl<P, D> ServerlessRepo for ServerlessRepoClient<P, D>
where
    P: ProvideAwsCredentials + 'static,
    D: DispatchSignedRequest + 'static,
{
    /// <p>Creates an application, optionally including an AWS SAM file to create the first application version in the same call.</p>
    fn create_application(
        &self,
        input: &CreateApplicationRequest,
    ) -> RusotoFuture<CreateApplicationResponse, CreateApplicationError> {
        let request_uri = "/applications";

        let mut request = SignedRequest::new("POST", "serverlessrepo", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(input).unwrap());
        request.set_payload(encoded);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 201 {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<CreateApplicationResponse>(&body).unwrap();

                    result
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(CreateApplicationError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Creates an application version.</p>
    fn create_application_version(
        &self,
        input: &CreateApplicationVersionRequest,
    ) -> RusotoFuture<CreateApplicationVersionResponse, CreateApplicationVersionError> {
        let request_uri = format!(
            "/applications/{application_id}/versions/{semantic_version}",
            application_id = input.application_id,
            semantic_version = input.semantic_version
        );

        let mut request = SignedRequest::new("PUT", "serverlessrepo", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(input).unwrap());
        request.set_payload(encoded);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 201 {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<CreateApplicationVersionResponse>(&body).unwrap();

                    result
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(CreateApplicationVersionError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Creates an AWS CloudFormation ChangeSet for the given application.</p>
    fn create_cloud_formation_change_set(
        &self,
        input: &CreateCloudFormationChangeSetRequest,
    ) -> RusotoFuture<CreateCloudFormationChangeSetResponse, CreateCloudFormationChangeSetError>
    {
        let request_uri = format!(
            "/applications/{application_id}/changesets",
            application_id = input.application_id
        );

        let mut request = SignedRequest::new("POST", "serverlessrepo", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(input).unwrap());
        request.set_payload(encoded);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 201 {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<CreateCloudFormationChangeSetResponse>(
                        &body,
                    ).unwrap();

                    result
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(CreateCloudFormationChangeSetError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Gets the specified application.</p>
    fn get_application(
        &self,
        input: &GetApplicationRequest,
    ) -> RusotoFuture<GetApplicationResponse, GetApplicationError> {
        let request_uri = format!(
            "/applications/{application_id}",
            application_id = input.application_id
        );

        let mut request = SignedRequest::new("GET", "serverlessrepo", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.semantic_version {
            params.put("semanticVersion", x);
        }
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<GetApplicationResponse>(&body).unwrap();

                    result
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(GetApplicationError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Gets the policy for the specified application.</p>
    fn get_application_policy(
        &self,
        input: &GetApplicationPolicyRequest,
    ) -> RusotoFuture<GetApplicationPolicyResponse, GetApplicationPolicyError> {
        let request_uri = format!(
            "/applications/{application_id}/policy",
            application_id = input.application_id
        );

        let mut request = SignedRequest::new("GET", "serverlessrepo", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<GetApplicationPolicyResponse>(&body).unwrap();

                    result
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(GetApplicationPolicyError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Lists versions for the specified application.</p>
    fn list_application_versions(
        &self,
        input: &ListApplicationVersionsRequest,
    ) -> RusotoFuture<ListApplicationVersionsResponse, ListApplicationVersionsError> {
        let request_uri = format!(
            "/applications/{application_id}/versions",
            application_id = input.application_id
        );

        let mut request = SignedRequest::new("GET", "serverlessrepo", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_items {
            params.put("maxItems", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("nextToken", x);
        }
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<ListApplicationVersionsResponse>(&body).unwrap();

                    result
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ListApplicationVersionsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Lists applications owned by the requester.</p>
    fn list_applications(
        &self,
        input: &ListApplicationsRequest,
    ) -> RusotoFuture<ListApplicationsResponse, ListApplicationsError> {
        let request_uri = "/applications";

        let mut request = SignedRequest::new("GET", "serverlessrepo", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_items {
            params.put("maxItems", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("nextToken", x);
        }
        request.set_params(params);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<ListApplicationsResponse>(&body).unwrap();

                    result
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ListApplicationsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Puts the policy for the specified application.</p>
    fn put_application_policy(
        &self,
        input: &PutApplicationPolicyRequest,
    ) -> RusotoFuture<PutApplicationPolicyResponse, PutApplicationPolicyError> {
        let request_uri = format!(
            "/applications/{application_id}/policy",
            application_id = input.application_id
        );

        let mut request = SignedRequest::new("PUT", "serverlessrepo", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(input).unwrap());
        request.set_payload(encoded);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<PutApplicationPolicyResponse>(&body).unwrap();

                    result
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(PutApplicationPolicyError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Updates the specified application.</p>
    fn update_application(
        &self,
        input: &UpdateApplicationRequest,
    ) -> RusotoFuture<UpdateApplicationResponse, UpdateApplicationError> {
        let request_uri = format!(
            "/applications/{application_id}",
            application_id = input.application_id
        );

        let mut request = SignedRequest::new("PATCH", "serverlessrepo", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(input).unwrap());
        request.set_payload(encoded);

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<UpdateApplicationResponse>(&body).unwrap();

                    result
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(UpdateApplicationError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }
}

#[cfg(test)]
mod protocol_tests {}
