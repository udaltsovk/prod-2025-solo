use crate::common::environment::with_test_environment;
use actix_http::StatusCode;
use common::
     models::{
        builders::{Buildable, Builder}, Advertiser, Campaign, CampaignUpdate, CreateCampaign, Model, Targeting
    };
use rand::{rng, seq::IndexedRandom, Rng};
use rstest::rstest;
use uuid::Uuid;
use validator::ValidateLength;

mod common;

#[rstest]
#[case::impressions_limit(
    CreateCampaign::builder()
        .with_impressions_limit(-1)
        .build()
)]
#[case::clicks_limit(
    CreateCampaign::builder()
        .with_clicks_limit(-1)
        .build()
)]
#[case::clicks_limit_greater_than_impressions_limit(
    CreateCampaign::builder()
        .with_impressions_limit(10_000)
        .with_clicks_limit(100_000)
        .build()
)]
#[case::cost_per_impression(
    CreateCampaign::builder()
        .with_cost_per_impression(-1.0)
        .build()
)]
#[case::cost_per_click(
    CreateCampaign::builder()
        .with_cost_per_click(-1.0)
        .build()
)]
#[case::start_date(
    CreateCampaign::builder()
        .with_start_date(-1)
        .build()
)]
#[case::end_date(
    CreateCampaign::builder()
        .with_end_date(-1)
        .build()
)]
#[case::end_date_earlier_than_start_date(
    CreateCampaign::builder()
        .with_start_date(10)
        .with_end_date(7)
        .build()
)]
#[case::targeting_gender(
    CreateCampaign::builder()
        .with_targeting(
            Targeting::builder()
                .with_gender(Some("baobab"))
                .build()
        )
        .build()
)]
#[case::targeting_age_from(
    CreateCampaign::builder()
        .with_targeting(
            Targeting::builder()
                .with_age_from(Some(-1))
                .build(),
        )
        .build()
)]
#[case::targeting_age_to(
    CreateCampaign::builder()
        .with_targeting(
            Targeting::builder()
                .with_age_from(Some(-1))
                .build(),
        )
        .build()
)]
#[case::targeting_age_from_greater_than_age_to(
    CreateCampaign::builder()
        .with_targeting(
            Targeting::builder()
                .with_age_from(Some(100))
                .with_age_to(Some(20))
                .build(),
        )
        .build()
)]
#[case::profanity_ad_title(
    CreateCampaign::builder()
        .with_ad_title("Курсы по питону")
        .build()
)]
#[case::profanity_ad_text(
    CreateCampaign::builder()
        .with_ad_text("Научим вас кодить на питоне за 3 недели!")
        .build()
)]
#[case::profanity_targeting_location(
    CreateCampaign::builder()
        .with_targeting(
            Targeting::builder()
            .with_location(Some("питон"))
            .build()
        )
        .build()
)]
#[actix_rt::test]
async fn create_campaign_invalid(#[case] campaign: CreateCampaign) {
    with_test_environment(|test_env| async move {
        let api = test_env.api;

        let advertiser = Advertiser::new();
        let resp = api.upsert_advertisers(vec![advertiser.clone()]).await;
        assert_status!(resp, StatusCode::CREATED);

        let resp = api
            .create_campaign(advertiser.id, campaign)
            .await;
        assert_status!(resp, StatusCode::BAD_REQUEST);
    })
    .await
}

#[actix_rt::test]
async fn create_campaign_non_existent() {
    with_test_environment(|test_env| async move {
        let api = test_env.api;

        let campaign = CreateCampaign::new();

        let resp = api
            .create_campaign(Uuid::now_v7(), campaign)
            .await;
        assert_status!(resp, StatusCode::NOT_FOUND);
    })
    .await
}

#[actix_rt::test]
async fn create_campaign() {
    with_test_environment(|test_env| async move {
        let api = test_env.api;

        let advertiser = Advertiser::new();
        let resp = api.upsert_advertisers(vec![advertiser.clone()]).await;
        assert_status!(resp, StatusCode::CREATED);

        let campaign = Campaign::builder()
            .with_advertiser_id(advertiser.id)
            .build();

        let resp = api
            .create_campaign(advertiser.id, campaign.inner.clone())
            .await;
        assert_json!(resp, StatusCode::CREATED, Campaign, campaign);
    })
    .await
}

#[actix_rt::test]
async fn list_campaigns_non_existent_advertiser() {
    with_test_environment(|test_env| async move {
        let api = test_env.api;

        let resp = api
            .list_campaigns(Uuid::now_v7(), None, None)
            .await;
        assert_status!(resp, StatusCode::NOT_FOUND);
    })
    .await
}

#[rstest]
#[case::size(
    Some(-1),
    None
)]
#[case::page(
    None,
    Some(-1)
)]
#[actix_rt::test]
async fn list_campaigns_invalid_pagination(#[case] size: Option<i64>, #[case] page: Option<i64>) {
    with_test_environment(|test_env| async move {
        let api = test_env.api;

        let advertiser = Advertiser::new();
        let resp = api.upsert_advertisers(vec![advertiser.clone()]).await;
        assert_status!(resp, StatusCode::CREATED);

        let resp = api
            .list_campaigns(advertiser.id, size, page)
            .await;
        assert_status!(resp, StatusCode::BAD_REQUEST);
    })
    .await
}

#[rstest]
#[case::no_pagination(
    None, 
    None
)]
#[case::size(
    Some(rng().random_range(1..57)), 
    None
)]
#[case::size_0(
    Some(0), 
    None
)]
#[case::big_size(
    Some(75), 
    None
)]
#[case::page(
    None, 
    Some(rng().random_range(0..=8))
)]
#[case::big_page(
    None, 
    Some(57)
)]
#[case::size_page(
    Some(5), 
    Some(7)
)]
#[actix_rt::test]
async fn list_campaigns(#[case] size: Option<i64>, #[case] page: Option<i64>) {
    with_test_environment(|test_env| async move {
        let api = test_env.api;

        let advertiser = Advertiser::new();
        let resp = api.upsert_advertisers(vec![advertiser.clone()]).await;
        assert_status!(resp, StatusCode::CREATED);

        let campaigns: Vec<Campaign> = (0..100)
            .map(|_| {
                Campaign::builder()
                    .with_advertiser_id(advertiser.id)
                    .build()
            })
            .collect();
        for campaign in campaigns.clone() {
            let resp = api.create_campaign(advertiser.id, campaign.inner)
                .await;
            assert_status!(resp, StatusCode::CREATED);
        }

        let size_expected = size.unwrap_or(7) as usize;
        let page_expected = page.unwrap_or(0) as usize;

        let expected: Vec<Campaign> = if size_expected == 0 {
            vec![]
        } else {
            let paginated = campaigns.chunks(size_expected.min(57)).collect::<Vec<_>>();
            if page_expected as u64 > paginated.length().unwrap() {
                vec![]
            } else {
                paginated[page_expected].to_vec()
            }
        };

        let resp = api
            .list_campaigns(advertiser.id, size, page)
            .await;
        assert_json!(resp, StatusCode::OK, Vec<Campaign>, expected);
    })
    .await
}

#[actix_rt::test]
async fn list_campaigns_empty() {
    with_test_environment(|test_env| async move {
        let api = test_env.api;

        let advertisers: Vec<Advertiser> = (0..100).map(|_| Advertiser::new()).collect();
        let advertiser = advertisers.choose(&mut rng()).unwrap().clone();
        let resp = api.upsert_advertisers(advertisers.clone()).await;
        assert_status!(resp, StatusCode::CREATED);

        let campaigns: Vec<Campaign> = advertisers
            .iter()
            .filter_map(|a| {
                if a.id == advertiser.id {
                    None
                } else {
                    Some(Campaign::builder().with_advertiser_id(a.id).build())
                }
            })
            .collect();
        for campaign in campaigns {
            let resp = api.create_campaign(campaign.advertiser_id, campaign.inner)
                .await;
            assert_status!(resp, StatusCode::CREATED);
        }

        let resp = api
            .list_campaigns(advertiser.id, None, None)
            .await;
        assert_status!(resp, StatusCode::OK);

        let campaigns_resp: Vec<Campaign> = Vec::from_resp(resp).await;
        assert_eq!(campaigns_resp.length(), Some(0));
    })
    .await
}

#[actix_rt::test]
async fn get_campaign_by_id_non_existent() {
    with_test_environment(|test_env| async move {
        let api = test_env.api;

        let resp = api
            .get_campaign_by_id(Uuid::now_v7(), Uuid::now_v7())
            .await;
        assert_status!(resp, StatusCode::NOT_FOUND);

        let advertiser = Advertiser::new();
        api.upsert_advertisers(vec![advertiser.clone()]).await;

        let resp = api
            .get_campaign_by_id(advertiser.id, Uuid::now_v7())
            .await;
        assert_status!(resp, StatusCode::NOT_FOUND);
    })
    .await
}

#[actix_rt::test]
async fn get_campaign_by_id() {
    with_test_environment(|test_env| async move {
        let api = test_env.api;

        let advertiser = Advertiser::new();
        let resp = api.upsert_advertisers(vec![advertiser.clone()]).await;
        assert_status!(resp, StatusCode::CREATED);

        let campaigns: Vec<CreateCampaign> = (0..100).map(|_| CreateCampaign::new()).collect();
        let campaign = campaigns.choose(&mut rng()).unwrap().clone();

        for c in campaigns {
            if c == campaign {
                continue;
            }
            let resp = api.create_campaign(advertiser.id, c).await;
            assert_status!(resp, StatusCode::CREATED);
        }

        let resp = api
            .create_campaign(advertiser.id, campaign.clone())
            .await;
        assert_status!(resp, StatusCode::CREATED);

        let campaign_resp = Campaign::from_resp(resp).await;

        let resp = api
            .get_campaign_by_id(advertiser.id, campaign_resp.id)
            .await;
        assert_json!(resp, StatusCode::OK, Campaign, campaign_resp);
    })
    .await
}

#[rstest]
#[case::impressions_limit(
    CampaignUpdate::builder()
        .with_impressions_limit(Some(-1))
        .build()
)]
#[case::clicks_limit(
    CampaignUpdate::builder()
        .with_clicks_limit(Some(-1))
        .build()
)]
#[case::clicks_limit_greater_than_impressions_limit(
    CampaignUpdate::builder()
        .with_impressions_limit(Some(10_000))
        .with_clicks_limit(Some(100_000))
        .build()
)]
#[case::cost_per_impression(
    CampaignUpdate::builder()
        .with_cost_per_impression(Some(-1.0))
        .build()
)]
#[case::cost_per_click(
    CampaignUpdate::builder()
        .with_cost_per_click(Some(-1.0))
        .build()
)]
#[case::start_date(
    CampaignUpdate::builder()
        .with_start_date(Some(-1))
        .build()
)]
#[case::end_date(
    CampaignUpdate::builder()
        .with_end_date(Some(-1))
        .build()
)]
#[case::end_date_earlier_than_start_date(
    CampaignUpdate::builder()
        .with_start_date(Some(10))
        .with_end_date(Some(7))
        .build()
)]
#[case::targeting_gender(
    CampaignUpdate::builder()
        .with_targeting(Some(
            Targeting::builder()
                .with_gender(Some("baobab"))
                .build()
        ))
        .build()
)]
#[case::targeting_age_from(
    CampaignUpdate::builder()
        .with_targeting(Some(
            Targeting::builder()
                .with_age_from(Some(-1))
                .build(),
        ))
        .build()
)]
#[case::targeting_age_to(
    CampaignUpdate::builder()
        .with_targeting(Some(
            Targeting::builder()
                .with_age_from(Some(-1))
                .build(),
        ))
        .build()
)]
#[case::targeting_age_from_greater_than_age_to(
    CampaignUpdate::builder()
        .with_targeting(Some(
            Targeting::builder()
                .with_age_from(Some(100))
                .with_age_to(Some(20))
                .build(),
        ))
        .build()
)]
#[case::profanity_ad_title(
    CampaignUpdate::builder()
        .with_ad_title(Some("Курсы по питону"))
        .build()
)]
#[case::profanity_ad_text(
    CampaignUpdate::builder()
        .with_ad_text(Some("Научим вас кодить на питоне за 3 недели!"))
        .build()
)]
#[case::profanity_targeting_location(
    CampaignUpdate::builder()
        .with_targeting(Some(
            Targeting::builder()
                .with_location(Some("Питон"))
                .build()
        ))
        .build()
)]
#[actix_rt::test]
async fn update_campaign_invalid(#[case] campaign_update: CampaignUpdate) {
    with_test_environment(|test_env| async move {
        let api = test_env.api;

        let resp = api
            .update_campaign(
                Uuid::now_v7(),
                Uuid::now_v7(),
                campaign_update,
            )
            .await;
        assert_status!(resp, StatusCode::BAD_REQUEST);
    })
    .await
}

#[actix_rt::test]
async fn update_campaign_non_existent() {
    with_test_environment(|test_env| async move {
        let api = test_env.api;

        let resp = api
            .update_campaign(
                Uuid::now_v7(),
                Uuid::now_v7(),
                CampaignUpdate::new(),
            )
            .await;
        assert_status!(resp, StatusCode::NOT_FOUND);

        let advertiser = Advertiser::new();
        let resp = api.upsert_advertisers(vec![advertiser.clone()]).await;
        assert_status!(resp, StatusCode::CREATED);

        let resp = api
            .update_campaign(
                advertiser.id,
                Uuid::now_v7(),
                CampaignUpdate::new(),
            )
            .await;
        assert_status!(resp, StatusCode::NOT_FOUND);
    })
    .await
}

#[rstest]
#[case::impressions_limit(
    CampaignUpdate::builder()
        .with_impressions_limit(Some(100_000))
        .build()
)]
#[case::clicks_limit(
    CampaignUpdate::builder()
        .with_clicks_limit(Some(0))
        .build()
)]
#[case::start_date(
    CampaignUpdate::builder()
        .with_start_date(Some(1))
        .build()
)]
#[case::end_date(
    CampaignUpdate::builder()
        .with_end_date(Some(100))
        .build()
)]
#[case::targeting(
    CampaignUpdate::builder()
        .with_targeting(Some(
            Targeting::new()
        ))
        .build()
)]
#[actix_rt::test]
async fn update_campaign_started(#[case] campaign_update: CampaignUpdate) {
    with_test_environment(|test_env| async move {
        let api = test_env.api;

        let advertiser = Advertiser::new();
        let resp = api.upsert_advertisers(vec![advertiser.clone()]).await;
        assert_status!(resp, StatusCode::CREATED);

        let campaign = CreateCampaign::builder()
            .with_start_date(1)
            .build();

        let resp = api
            .create_campaign(advertiser.id, campaign.clone())
            .await;
        assert_status!(resp, StatusCode::CREATED);

        let campaign_resp = Campaign::from_resp(resp).await;

        let resp = api
            .update_campaign(advertiser.id, campaign_resp.id, campaign_update)
            .await;
        assert_status!(resp, StatusCode::CONFLICT);
    })
    .await
}

#[actix_rt::test]
async fn update_campaign() {
    with_test_environment(|test_env| async move {
        let api = test_env.api;

        let advertiser = Advertiser::new();
        let resp = api.upsert_advertisers(vec![advertiser.clone()]).await;
        assert_status!(resp, StatusCode::CREATED);

        let campaign = Campaign::builder()
            .with_advertiser_id(advertiser.id)
            .with_inner(
                CreateCampaign::builder()
                    .with_start_date(10)
                    .build()
            )
            .build();

        let resp = api
            .create_campaign(advertiser.id, campaign.inner.clone())
            .await;
        assert_status!(resp, StatusCode::CREATED);

        let campaign_resp = Campaign::from_resp(resp).await;
        let campaign_update = CampaignUpdate::new();

        let resp = api
            .update_campaign(advertiser.id, campaign_resp.id, campaign_update.clone())
            .await;
        assert_json!(
            resp,         
            StatusCode::OK,
            Campaign,
            campaign.update(campaign_update)
        );
    })
    .await
}

#[actix_rt::test]
async fn delete_campaign_non_existent() {
    with_test_environment(|test_env| async move {
        let api = test_env.api;

        let resp = api
            .delete_campaign(Uuid::now_v7(), Uuid::now_v7())
            .await;
        assert_status!(resp, StatusCode::NOT_FOUND);

        let advertiser = Advertiser::new();
        let resp = api.upsert_advertisers(vec![advertiser.clone()]).await;
        assert_status!(resp, StatusCode::CREATED);

        let resp = api
            .delete_campaign(advertiser.id, Uuid::now_v7())
            .await;
        assert_status!(resp, StatusCode::NOT_FOUND);
    })
    .await
}

#[actix_rt::test]
async fn delete_campaign() {
    with_test_environment(|test_env| async move {
        let api = test_env.api;

        let advertiser = Advertiser::new();
        let resp = api.upsert_advertisers(vec![advertiser.clone()]).await;
        assert_status!(resp, StatusCode::CREATED);

        let campaigns: Vec<CreateCampaign> = (0..100).map(|_| CreateCampaign::new()).collect();
        let campaign = campaigns.choose(&mut rng()).unwrap().clone();

        for c in campaigns {
            if c == campaign {
                continue;
            }
            let resp = api.create_campaign(advertiser.id, c).await;
            assert_status!(resp, StatusCode::CREATED);
        }

        let resp = api
            .create_campaign(advertiser.id, campaign.clone())
            .await;
        assert_status!(resp, StatusCode::CREATED);

        let campaign_resp = Campaign::from_resp(resp).await;

        let resp = api
            .delete_campaign(advertiser.id, campaign_resp.id)
            .await;
        assert_status!(resp, StatusCode::NO_CONTENT);
    })
    .await
}
