use crate::infrastructure::{
    dto::report::{GetReportRequest, QueryReportsRequest, ReportDetail, ReportWinRate},
    error::ERROR_FORGE_CLIENT_ERROR,
    gateway,
    shared::{QueryResult, RespResult},
};

#[tauri::command]
pub async fn query_reports<'a>(req: QueryReportsRequest) -> RespResult<'a, QueryResult<ReportDetail>> {
    if let Some(client) = gateway::client() {
        return Ok(client.query_reports(req).await?);
    }

    Err(ERROR_FORGE_CLIENT_ERROR)
}

#[tauri::command]
pub async fn get_report<'a>(req: GetReportRequest) -> RespResult<'a, ReportDetail> {
    if let Some(client) = gateway::client() {
        return Ok(client.get_report(req).await?);
    }

    Err(ERROR_FORGE_CLIENT_ERROR)
}

#[tauri::command]
pub async fn get_report_win_rate<'a>(req: QueryReportsRequest) -> RespResult<'a, ReportWinRate> {
    if let Some(client) = gateway::client() {
        return Ok(client.get_report_win_rate(req).await?);
    }

    Err(ERROR_FORGE_CLIENT_ERROR)
}
