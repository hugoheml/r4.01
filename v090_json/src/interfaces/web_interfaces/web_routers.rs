#[derive(Clone)]
pub struct WebRoutes {
	pub index: &'static str,
	pub results: &'static str,
	pub vote: &'static str,
	pub json: &'static str,
	pub v1: &'static str
}

pub const WEB_ROUTES: WebRoutes = WebRoutes {
	index: "/",
	results: "/results",
	vote: "/vote",
	json: "/json",
	v1: "/v1"
};