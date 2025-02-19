use std::net::SocketAddr;

use anyhow::Ok;
use axum::Router;
use async_trait::async_trait;

use crate::{interfaces::{lexicon::Lexicon, web_interfaces::{router::make_router, web_routers::WEB_ROUTES, AxumState}}, service::Service, storage::Storage, use_cases::VotingController};

pub struct WebService {
	address: SocketAddr,
	router: Router
}

#[async_trait]
impl <Store: Storage + Send + Sync + Clone + 'static> Service<Store> for WebService {
	fn new(port: u16, lexicon: Lexicon, controller: VotingController<Store>) -> Self {
		// make_router
		let address = format!("0.0.0.0:{}", port).parse().unwrap();

		let axum_state = AxumState::<Store>{
			controller: controller,
			routes: WEB_ROUTES,
			lexicon: lexicon
		};

		let router = make_router(axum_state, &WEB_ROUTES);

		WebService {
			address,
			router,
		}
	}
	async fn serve(&mut self) -> Result<(), anyhow::Error> {
		let listener = tokio::net::TcpListener::bind(&self.address).await.unwrap();
		axum::serve(listener, self.router.clone()).await.unwrap();
		Ok(())
	}
}