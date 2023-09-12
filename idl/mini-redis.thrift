namespace rs volo.example

struct PingRequest {
	1: required i64 id,
}

struct PingResponse {

}

struct SetRequest {
	1: required i64 id,
	2: required string key,
	3: required string value,
}

struct SetResponse {
	1: required bool res,
}

struct GetRequest {
	1: required i64 id,
	2: required string key,
}

struct GetResponse {
	1: optional string value,
}

struct DelRequest {
	1: required i64 id,
	2: required list<string> keys, 
}

struct DelResponse {
	1: required i64 deleted,
}

struct PubRequest {
	1: required i64 id,
	2: required string channel,
	3: required string msg,
}

struct PubResponse {
	1: required i64 num,
}

struct SubRequest {
	1: required i64 id,
	2: required string channel,
}

struct SubResponse {
	1: required string msg,
}

service RedisService {
	PingResponse Ping (1: PingRequest ping),
	SetResponse Set (1: SetRequest setreq),
	GetResponse Get (1: GetRequest getreq),
	DelResponse Del (1: DelRequest delreq),
	PubResponse Publish (1: PubRequest pubreq),
	SubResponse Sub (1: SubRequest subreq),
}