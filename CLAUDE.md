# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Atmos is an intelligent ambient lighting control system that analyzes web content to automatically adjust room lighting via Nature Remo smart home devices. The system consists of a browser extension (Svelte + WXT) that extracts content from web pages and a Rust backend server that processes this content to calculate an "atmosphere score" and control lighting.

## Architecture

The project follows **Hexagonal Architecture** (Domain-driven design):

- **Domain Layer**: Core business logic for atmosphere calculation (`AtmosFreq`, `AdjustLightingRequest`)
- **Inbound Ports**: HTTP API endpoints (`GET/POST /lighting`)
- **Outbound Ports**: Nature Remo API integration via custom `remo_api` crate
- **Tech Stack**: Rust (Axum) backend, Svelte 5 + WXT extension, TypeScript throughout

## Development Commands

### Main Development
- `task dev` - Start both extension and server concurrently
- `task gen` - Generate TypeScript types from Rust and API client from OpenAPI

### Extension (client/atmos_extension/)
- `task extension:dev` - Start extension development mode (`pnpm dev`)
- `task extension:format-w` - Format code with Biome (`pnpm biome format --write`)
- `task extension:lint-w` - Lint with Biome (`pnpm biome lint --apply`)
- `task extension:check-w` - Run full Biome check with auto-fix
- `task extension:gen-api` - Generate API client from server's OpenAPI spec

### Server (server/)
- `task server:dev` - Start Rust server (`cargo run`)
- `task server:test` - Run all workspace tests (`cargo test --workspace`)
- `task server:gen-types` - Generate TypeScript types for extension

### Docker
- `task up` - Start Docker containers (`docker compose up -d --build`)
- `task down` - Stop Docker containers

## Code Generation Flow

1. Rust structs with `#[derive(TS)]` generate TypeScript types
2. Server exports OpenAPI spec at `/api-docs/openapi.json`
3. Extension generates type-safe API client with `openapi-zod-client`
4. Run `task gen` after backend model changes to sync types

## Testing

- **Server**: `task server:test` runs Cargo workspace tests
- **Extension**: Uses `svelte-check` for TypeScript validation
- **Linting**: Biome for extension, Rustfmt for server

## Key Directories

- `server/crates/atmos/` - Main server binary
- `server/crates/atmos_server/` - Core business logic and domain models
- `server/crates/remo_api/` - Nature Remo API client
- `client/atmos_extension/src/api/` - Generated API client and types
- `client/atmos_extension/src/entrypoints/` - Extension entry points

## Development Notes

- Uses pnpm for package management (extension)
- All documentation and comments are in Japanese
- Project uses Taskfile instead of Makefile for task automation
- Extension targets multiple browsers via WXT framework
- Type safety maintained across Rust-TypeScript boundary
