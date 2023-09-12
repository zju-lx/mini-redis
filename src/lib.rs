#![feature(impl_trait_in_assoc_type)]
use anyhow::{Error, Ok, anyhow};
use lazy_static::lazy_static;
use pilota::FastStr;
use std::collections::{VecDeque, HashMap};
use std::sync::{Arc, Mutex};
use volo_gen::volo::example::{
	PingRequest,
	PingResponse,
	SetRequest,
	SetResponse,
	GetRequest,
	GetResponse,
	DelRequest,
	DelResponse,
	PubRequest,
	PubResponse,
	SubRequest,
	SubResponse,
};
use async_channel::{Sender, Receiver, bounded};

lazy_static!(
	static ref HASHMAP: Mutex<HashMap<pilota::FastStr, pilota::FastStr>> = {
		let mut m = HashMap::new();
		Mutex::new(m)
	};

	static ref CHANNEL: Arc<(Sender<String>, Receiver<String>)> = {
        let mut channels = Arc::new(bounded(1));
        channels
    };
);
pub struct S;
	

#[volo::async_trait]
impl volo_gen::volo::example::RedisService for S {
	async fn ping(&self, _ping: volo_gen::volo::example::PingRequest) -> ::core::result::Result<volo_gen::volo::example::PingResponse, ::volo_thrift::AnyhowError>{
					Ok(PingResponse{})
				}
async fn set(&self, _setreq: volo_gen::volo::example::SetRequest) -> ::core::result::Result<volo_gen::volo::example::SetResponse, ::volo_thrift::AnyhowError>{
					let key = _setreq.key;
					let value = _setreq.value;
					HASHMAP.lock().unwrap().insert(key, value);
					Ok(SetResponse{res: true})
				}
async fn get(&self, _getreq: volo_gen::volo::example::GetRequest) -> ::core::result::Result<volo_gen::volo::example::GetResponse, ::volo_thrift::AnyhowError>{
					let key = _getreq.key;
					if let Some(_value) = HASHMAP.lock().unwrap().get(&key){
						Ok(GetResponse{value: Some(_value.clone())})
					} else {
						Ok(GetResponse{value: None})
					}
				}
async fn del(&self, _delreq: volo_gen::volo::example::DelRequest) -> ::core::result::Result<volo_gen::volo::example::DelResponse, ::volo_thrift::AnyhowError>{
					let mut map = HASHMAP.lock().unwrap();
					let keys: Vec<pilota::FastStr> = _delreq.keys.into_iter().filter(|k| {map.get(k) != None}).clone().collect();
					let cnt = keys.len() as i64;
					for key in keys {
						map.remove(&key);
					}
					Ok(DelResponse { deleted: cnt})
				}
async fn publish(&self, _pubreq: PubRequest) -> ::core::result::Result<volo_gen::volo::example::PubResponse, ::volo_thrift::AnyhowError> {
					let (sender, receiver) = CHANNEL.as_ref();
					sender.send(String::from(_pubreq.msg)).await;
					
					Ok(PubResponse { num: 1 })
}
async fn sub(&self, _subreq: SubRequest) -> ::core::result::Result<volo_gen::volo::example::SubResponse, ::volo_thrift::AnyhowError> {
					
					let (sender, receiver) = CHANNEL.as_ref();
					let msg = receiver.recv().await.unwrap();

					Ok(SubResponse {msg: FastStr::new(msg)})
}
}


#[derive(Clone)]
pub struct FilterService<S>(S);

#[volo::service]
impl<Cx, Req, S> volo::Service<Cx, Req> for FilterService<S>
where
    Req: std::fmt::Debug + Send + 'static,
    S: Send + 'static + volo::Service<Cx, Req> + Sync,
    S::Response: std::fmt::Debug,
    S::Error: std::fmt::Debug,
    Cx: Send + 'static,
	anyhow::Error: Into<S::Error>
{
    async fn call(&self, cx: &mut Cx, req: Req) -> Result<S::Response, S::Error> {
		let req_str = format!("{:?}", req);
		//println!("{}", req_str);
        tracing::debug!("Received request {:?}", &req);
		if req_str.as_str().starts_with("Del") {
			let err = anyhow!("filtered");
			return Err(err.into()) 
		}
        let resp = self.0.call(cx, req).await;
        resp
    }
}


pub struct FilterLayer;

impl<S> volo::Layer<S> for FilterLayer {
    type Service = FilterService<S>;

    fn layer(self, inner: S) -> Self::Service {
        FilterService(inner)
    }
}
