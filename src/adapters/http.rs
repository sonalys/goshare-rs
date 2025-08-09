
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};

use crate::repositories::GroupRepository;
use crate::{application, domain, repositories};


#[derive(Deserialize)]
pub struct CreateGroupDto {
    name: String,
}

#[derive(Serialize)]
pub struct CreateGroupResp {
    id: Uuid,
}

#[derive(Deserialize)]
pub struct AddMemberDto {
    name: String,
}

#[derive(Serialize)]
pub struct AddMemberResp {
    id: Uuid,
}

#[derive(Deserialize)]
pub struct AddExpenseDto {
    paid_by: Uuid,
    amount_cents: i64,
    description: String,
    participants: Option<Vec<Uuid>>,
}

#[derive(Serialize)]
pub struct AddExpenseResp {
    id: Uuid,
}

#[derive(Serialize)]
pub struct BalanceResp {
    member: Uuid,
    balance_cents: i64,
}

pub async fn create_group<R: repositories::GroupRepository + 'static>(
    svc: web::Data<Arc<application::LedgerService<R>>>,
    dto: web::Json<CreateGroupDto>,
) -> impl Responder {
    match svc.create_group(dto.name.clone()).await {
        Ok(id) => HttpResponse::Ok().json(CreateGroupResp { id: id.0 }),
        Err(e) => HttpResponse::BadRequest().body(e.to_string()),
    }
}

pub async fn add_member<R: repositories::GroupRepository + 'static>(
    svc: web::Data<Arc<application::LedgerService<R>>>,
    path: web::Path<Uuid>,
    dto: web::Json<AddMemberDto>,
) -> impl Responder {
    let group_id = domain::groups::GroupId(path.into_inner());
    match svc.add_member(group_id, dto.name.clone()).await {
        Ok(id) => HttpResponse::Ok().json(AddMemberResp { id: id.0 }),
        Err(e) => HttpResponse::BadRequest().body(e.to_string()),
    }
}

pub async fn add_expense<R: repositories::GroupRepository + 'static>(
    svc: web::Data<Arc<application::LedgerService<R>>>,
    path: web::Path<Uuid>,
    dto: web::Json<AddExpenseDto>,
) -> impl Responder {
    let group_id = domain::groups::GroupId(path.into_inner());
    let participants = dto
        .participants
        .clone()
        .unwrap_or_default()
        .into_iter()
        .map(domain::members::MemberId)
        .collect();
    match svc
        .add_expense(
            group_id,
            domain::members::MemberId(dto.paid_by),
            dto.amount_cents,
            dto.description.clone(),
            participants,
        )
        .await
    {
        Ok(id) => HttpResponse::Ok().json(AddExpenseResp { id: id.0 }),
        Err(e) => HttpResponse::BadRequest().body(e.to_string()),
    }
}

pub async fn get_balances<R: repositories::GroupRepository + 'static>(
    svc: web::Data<Arc<application::LedgerService<R>>>,
    path: web::Path<Uuid>,
) -> impl Responder {
    let group_id = domain::groups::GroupId(path.into_inner());
    match svc.compute_balances(group_id).await {
        Ok(bals) => {
            let resp: Vec<BalanceResp> = bals
                .into_iter()
                .map(|b| BalanceResp {
                    member: b.member.0,
                    balance_cents: b.balance_cents,
                })
                .collect();
            HttpResponse::Ok().json(resp)
        }
        Err(e) => HttpResponse::BadRequest().body(e.to_string()),
    }
}

pub async fn run_server<R: GroupRepository + 'static>(ledger_service: Arc<application::LedgerService<R>>) -> Result<(), std::io::Error> {
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(ledger_service.clone()))
            .configure(configure_routes::<R>)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

pub fn configure_routes<R: GroupRepository + 'static>(cfg: &mut web::ServiceConfig) {
     cfg
        .route(
            "/groups", 
            web::post().to(create_group::<R>),
        )
        .route(
            "/groups/{group_id}/members",
            web::post().to(add_member::<R>),
        )
        .route(
            "/groups/{group_id}/expenses",
            web::post().to(add_expense::<R>),
        )
        .route(
            "/groups/{group_id}/balances",
            web::get().to(get_balances::<R>),
        );
}