use dropshot::test_util::objects_post;
use dropshot::test_util::read_json;
use dropshot::test_util::ClientTestContext;
use dropshot::HttpErrorResponseBody;
use dropshot::Method;
use http::StatusCode;
use omicron_common::api::external::VpcRouter;
use omicron_common::api::external::VpcRouterCreateParams;
use omicron_nexus::authn::external::spoof::HTTP_HEADER_OXIDE_AUTHN_SPOOF;

use omicron_common::api::external::IdentityMetadataCreateParams;
use omicron_common::api::external::Organization;
use omicron_common::api::external::OrganizationCreateParams;
use omicron_common::api::external::Project;
use omicron_common::api::external::ProjectCreateParams;
use omicron_common::api::external::Vpc;
use omicron_common::api::external::VpcCreateParams;

pub async fn create_organization(
    client: &ClientTestContext,
    organization_name: &str,
) -> Organization {
    let input = OrganizationCreateParams {
        identity: IdentityMetadataCreateParams {
            name: organization_name.parse().unwrap(),
            description: "an org".to_string(),
        },
    };
    let authn_header = http::HeaderValue::from_static(
        omicron_nexus::authn::TEST_USER_UUID_PRIVILEGED,
    );
    let uri = client.url("/organizations");
    let request = hyper::Request::builder()
        .header(HTTP_HEADER_OXIDE_AUTHN_SPOOF, authn_header)
        .method(Method::POST)
        .uri(uri)
        .body(serde_json::to_string(&input).unwrap().into())
        .expect("attempted to construct invalid test request");
    let mut response = client
        .make_request_with_request(request, StatusCode::CREATED)
        .await
        .expect("failed to make request");
    read_json::<Organization>(&mut response).await
}

pub async fn create_project(
    client: &ClientTestContext,
    organization_name: &str,
    project_name: &str,
) -> Project {
    objects_post(
        &client,
        format!("/organizations/{}/projects", &organization_name).as_str(),
        ProjectCreateParams {
            identity: IdentityMetadataCreateParams {
                name: project_name.parse().unwrap(),
                description: "a pier".to_string(),
            },
        },
    )
    .await
}

pub async fn create_vpc(
    client: &ClientTestContext,
    organization_name: &str,
    project_name: &str,
    vpc_name: &str,
) -> Vpc {
    objects_post(
        &client,
        format!(
            "/organizations/{}/projects/{}/vpcs",
            &organization_name, &project_name
        )
        .as_str(),
        VpcCreateParams {
            identity: IdentityMetadataCreateParams {
                name: vpc_name.parse().unwrap(),
                description: "vpc description".to_string(),
            },
            dns_name: "abc".parse().unwrap(),
        },
    )
    .await
}

// TODO: probably would be cleaner to replace these helpers with something that
// just generates the create params since that's the noisiest part
pub async fn create_vpc_with_error(
    client: &ClientTestContext,
    organization_name: &str,
    project_name: &str,
    vpc_name: &str,
    status: StatusCode,
) -> HttpErrorResponseBody {
    client
        .make_request_error_body(
            Method::POST,
            format!(
                "/organizations/{}/projects/{}/vpcs",
                &organization_name, &project_name
            )
            .as_str(),
            VpcCreateParams {
                identity: IdentityMetadataCreateParams {
                    name: vpc_name.parse().unwrap(),
                    description: String::from("vpc description"),
                },
                dns_name: "abc".parse().unwrap(),
            },
            status,
        )
        .await
}

pub async fn create_router(
    client: &ClientTestContext,
    organization_name: &str,
    project_name: &str,
    vpc_name: &str,
    router_name: &str,
) -> VpcRouter {
    objects_post(
        &client,
        format!(
            "/organizations/{}/projects/{}/vpcs/{}/routers",
            &organization_name, &project_name, &vpc_name
        )
        .as_str(),
        VpcRouterCreateParams {
            identity: IdentityMetadataCreateParams {
                name: router_name.parse().unwrap(),
                description: String::from("router description"),
            },
        },
    )
    .await
}
