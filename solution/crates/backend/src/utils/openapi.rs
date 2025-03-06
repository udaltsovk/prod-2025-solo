use utoipa::{openapi::OpenApi as OpenApiStruct, OpenApi};
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi, Debug)]
#[openapi(
    info(
        title = "PROD Backend 2025 Advertising Platform API",
        description = r#"API для управления данными клиентов, рекламодателей, рекламными кампаниями, показом объявлений, статистикой и управлением "текущим днём" в системе."#,
        version = "1.1.0"
    ),
    servers(
        (
            url = "http://localhost:8080",
            description = "Dev server"
        )
    ),
    tags(
        (
            name = "Clients",
            description = "Управление клиентами: создание и обновление информации о клиентах."
        ),
        (
            name = "Advertisers",
            description = "Управление рекламодателями и ML скорами для определения релевантности."
        ),
        (
            name = "Campaigns",
            description = "Управление рекламными кампаниями: создание, обновление, удаление и получение списка кампаний."
        ),
        (
            name = "Campaign images",
            description = "Управление изображениями рекламных кампаний: загрузка, обновление, удаление и получение изображения рекламной кампании."
        ),
        (
            name = "Ads",
            description = "Показ рекламных объявлений клиентам и фиксация кликов."
        ),
        (
            name = "Statistics",
            description = "Получение статистики по кампаниям и рекламодателям, а также ежедневной статистики."
        ),
        (
            name = "Time",
            description = "Управление текущим днём (эмуляция времени) в системе."
        )
    )
)]
pub struct Swagger;
impl Swagger {
    pub fn ui_service(api: OpenApiStruct) -> SwaggerUi {
        SwaggerUi::new("/swagger-ui/{_}*").url("/openapi.json", api)
    }
}
