use std::sync::Arc;

use async_graphql::SDLExportOptions;
use axum::Router;
use serde_json::json;
use utoipa::OpenApi;

use crate::global::Global;

mod gql;
mod rest;
pub mod docs;

pub fn routes(global: &Arc<Global>) -> Router<Arc<Global>> {
	Router::new().nest("/gql", gql::routes(global)).nest("/", rest::routes())
}

pub fn export_gql_schema() -> String {
	gql::schema(None).sdl_with_options(
		SDLExportOptions::default()
			.federation()
			.include_specified_by()
			.sorted_arguments()
			.sorted_enum_items()
			.sorted_fields(),
	)
}

pub fn docs() -> utoipa::openapi::OpenApi {
	docs::Docs::openapi()
}

pub fn export_openapi_json() -> String {
	let info = json!({
		"title": "7TV v4",
		"version": "4.0.0",
		"description": "Auto-generated v4 OpenAPI docs",
		"license": { "name": "Apache-2.0 with Commons Clause" }
	});

	// Minimal path entries for key v4 REST endpoints
	let mut paths = serde_json::Map::new();

	paths.insert(
		"/v4/auth/login".to_string(),
		json!({
			"get": {
				"summary": "Initiate login",
				"responses": { "302": { "description": "Redirect to provider" } }
			}
		}),
	);

	paths.insert(
		"/v4/auth/link".to_string(),
		json!({
			"get": {
				"summary": "Initiate account link",
				"responses": { "302": { "description": "Redirect to provider" } }
			}
		}),
	);

	paths.insert(
		"/v4/auth/login/finish".to_string(),
		json!({
			"post": {
				"summary": "Finish login",
				"requestBody": { "content": { "application/json": { "schema": { "type": "object" } } } },
				"responses": { "200": { "description": "Login success" } }
			}
		}),
	);

	paths.insert(
		"/v4/auth/link/finish".to_string(),
		json!({
			"post": {
				"summary": "Finish link",
				"requestBody": { "content": { "application/json": { "schema": { "type": "object" } } } },
				"responses": { "200": { "description": "Link success" } }
			}
		}),
	);

	paths.insert(
		"/v4/auth/logout".to_string(),
		json!({
			"post": {
				"summary": "Logout",
				"responses": { "204": { "description": "Logged out" } }
			}
		}),
	);

	paths.insert(
		"/v4/gql".to_string(),
		json!({
			"post": {
				"summary": "GraphQL endpoint",
				"requestBody": { "content": { "application/json": { "schema": { "type": "object" } } } },
				"responses": { "200": { "description": "GraphQL response" } }
			}
		}),
	);

	paths.insert(
		"/v4/gql/playground".to_string(),
		json!({
			"get": {
				"summary": "GraphQL playground",
				"responses": { "200": { "description": "GraphQL playground HTML" } }
			}
		}),
	);

	paths.insert(
		"/v4/emotes/".to_string(),
		json!({
			"post": {
				"summary": "Create emote",
				"requestBody": { "content": { "multipart/form-data": { "schema": { "type": "object" } } } },
				"responses": { "201": { "description": "Created" } }
			}
		}),
	);

	paths.insert(
		"/v4/badges/".to_string(),
		json!({
			"post": {
				"summary": "Create badge",
				"requestBody": { "content": { "multipart/form-data": { "schema": { "type": "object" } } } },
				"responses": { "201": { "description": "Created" } }
			}
		}),
	);

	paths.insert(
		"/v4/events/create".to_string(),
		json!({
			"post": {
				"summary": "Create event",
				"requestBody": { "content": { "application/json": { "schema": { "type": "object" } } } },
				"responses": { "200": { "description": "Success" } }
			}
		}),
	);

	paths.insert(
		"/v4/users/{id}/profile-picture".to_string(),
		json!({
			"post": {
				"summary": "Upload profile picture",
				"parameters": [{ "name": "id", "in": "path", "required": true, "schema": { "type": "string" } }],
				"requestBody": { "content": { "image/*": { "schema": { "type": "string", "format": "binary" } } } },
				"responses": { "200": { "description": "Uploaded" } }
			}
		}),
	);

	paths.insert(
		"/v4/users/{id}/products".to_string(),
		json!({
			"get": {
				"summary": "Get user products",
				"parameters": [{ "name": "id", "in": "path", "required": true, "schema": { "type": "string" } }],
				"responses": { "200": { "description": "OK" } }
			}
		}),
	);

	let openapi = json!({
		"openapi": "3.0.3",
		"info": info,
		"paths": serde_json::Value::Object(paths),
	});

	openapi.to_string()
}
