# CODEBASE.md: imgbb-tui Semantic Digest

> **Notice**: AI-optimized semantic index. Do not write narrative prose. Keep token density high.

## 1. System Topology & Data Flow
```text
Entrypoint ──> CLI/Parser ──> Domain Logic ──> Infra/IO
```

## 2. Global Constraints & Architecture Patterns
- **Primary Language**: Rust 2021 edition
- **Architectural Paradigm**: Role-based (domain/, infra/, api/cli/, tui/)
- **Hard Constraints**: <400 lines/file, <60 lines/fn, zero production unwrap(), 0 warnings.
- **Target Distribution**: Linux x86_64 standalone binary

## 3. Module & Interface Skeleton

### `src/api/imgbb.rs` (Role: api, Lines: 121)
- **Responsibility**: Core api logic in src/api/imgbb.rs
- **Imports**: use crate :: domain :: error :: AppError , use crate :: domain :: model :: { ImagePayload , UploadResult } , use base64 :: Engine , use serde :: Deserialize , use std :: time :: Duration 
- **Types & Enums**:
  ```rust
  pub struct ImgbbClient
  ```
- **Public Functions & Signatures**:
  ```rust
  fn new (api_key : String) -> Result < Self , AppError >
  async fn upload (& self , payload : & ImagePayload) -> Result < UploadResult , AppError >
  ```

### `src/api.rs` (Role: api, Lines: 1)
- **Responsibility**: Core api logic in src/api.rs

### `src/cli/app.rs` (Role: cli, Lines: 275)
- **Responsibility**: Core cli logic in src/cli/app.rs
- **Imports**: use crate :: api :: imgbb :: ImgbbClient , use crate :: cli :: events :: { AppEvent , EventLoop } , use crate :: domain :: error :: AppError , use crate :: domain :: model :: { HistoryEntry , ImagePayload , UploadResult , UploadState } , use crate :: infra :: clipboard , use crate :: infra :: config :: Config , use crate :: infra :: history :: HistoryStore , use crate :: ui :: render , use anyhow :: Result , use crossterm :: event :: { KeyCode , KeyEvent , KeyModifiers } , use crossterm :: execute , use crossterm :: terminal :: { disable_raw_mode , enable_raw_mode , EnterAlternateScreen , LeaveAlternateScreen , } , use ratatui :: backend :: CrosstermBackend , use ratatui :: Terminal , use std :: io :: Stdout , use std :: time :: { SystemTime , UNIX_EPOCH } , use tokio :: sync :: mpsc 
- **Types & Enums**:
  ```rust
  pub struct App
  ```
- **Public Functions & Signatures**:
  ```rust
  fn new () -> Result < Self >
  async fn run (& mut self) -> Result < () >
  fn input (& self) -> & str
  fn state (& self) -> UploadState
  fn result (& self) -> Option < & UploadResult >
  fn error (& self) -> Option < & str >
  fn status (& self) -> Option < & str >
  fn needs_key_setup (& self) -> bool
  fn spinner_frame (& self) -> usize
  ```

### `src/cli/events.rs` (Role: cli, Lines: 35)
- **Responsibility**: Core cli logic in src/cli/events.rs
- **Imports**: use crossterm :: event :: { Event , EventStream , KeyEvent } , use futures :: StreamExt 
- **Types & Enums**:
  ```rust
  pub enum AppEvent
  pub struct EventLoop
  ```
- **Public Functions & Signatures**:
  ```rust
  fn new () -> Self
  async fn next (& mut self) -> Option < AppEvent >
  ```

### `src/cli.rs` (Role: cli, Lines: 2)
- **Responsibility**: Core cli logic in src/cli.rs

### `src/domain/error.rs` (Role: domain, Lines: 19)
- **Responsibility**: Core domain logic in src/domain/error.rs
- **Imports**: use thiserror :: Error 
- **Types & Enums**:
  ```rust
  pub enum AppError
  ```

### `src/domain/model.rs` (Role: domain, Lines: 52)
- **Responsibility**: Core domain logic in src/domain/model.rs
- **Imports**: use serde :: { Deserialize , Serialize } , use std :: path :: PathBuf 
- **Types & Enums**:
  ```rust
  pub struct ImagePayload
  pub struct UploadResult
  pub struct HistoryEntry
  pub enum UploadState
  ```
- **Public Functions & Signatures**:
  ```rust
  fn from_file (path : & PathBuf) -> Result < Self , crate :: domain :: error :: AppError >
  fn size (& self) -> u64
  ```

### `src/domain.rs` (Role: domain, Lines: 2)
- **Responsibility**: Core domain logic in src/domain.rs

### `src/infra/clipboard.rs` (Role: infra, Lines: 58)
- **Responsibility**: Core infra logic in src/infra/clipboard.rs
- **Imports**: use crate :: domain :: error :: AppError , use std :: io :: Write , use std :: process :: { Command , Stdio } 
- **Public Functions & Signatures**:
  ```rust
  fn read_clipboard_image () -> Result < Vec < u8 > , AppError >
  fn copy_to_clipboard (text : & str) -> Result < () , AppError >
  ```

### `src/infra/config.rs` (Role: infra, Lines: 49)
- **Responsibility**: Core infra logic in src/infra/config.rs
- **Imports**: use crate :: domain :: error :: AppError , use directories :: ProjectDirs , use serde :: { Deserialize , Serialize } , use std :: path :: PathBuf 
- **Types & Enums**:
  ```rust
  pub struct Config
  ```
- **Public Functions & Signatures**:
  ```rust
  fn project_dirs () -> Result < ProjectDirs , AppError >
  fn path () -> Result < PathBuf , AppError >
  fn load () -> Result < Self , AppError >
  fn save (& self) -> Result < () , AppError >
  fn needs_api_key (& self) -> bool
  ```

### `src/infra/history.rs` (Role: infra, Lines: 37)
- **Responsibility**: Core infra logic in src/infra/history.rs
- **Imports**: use crate :: domain :: error :: AppError , use crate :: domain :: model :: HistoryEntry , use std :: path :: PathBuf 
- **Types & Enums**:
  ```rust
  pub struct HistoryStore
  ```
- **Public Functions & Signatures**:
  ```rust
  fn new () -> Result < Self , AppError >
  fn load (& self) -> Vec < HistoryEntry >
  fn push (& self , entry : HistoryEntry) -> Result < () , AppError >
  ```

### `src/infra.rs` (Role: infra, Lines: 3)
- **Responsibility**: Core infra logic in src/infra.rs

### `src/main.rs` (Role: general, Lines: 14)
- **Responsibility**: Core general logic in src/main.rs
- **Imports**: use anyhow :: Result , use cli :: app :: App 

### `src/ui/render.rs` (Role: tui, Lines: 300)
- **Responsibility**: Core tui logic in src/ui/render.rs
- **Imports**: use crate :: cli :: app :: App , use crate :: domain :: model :: UploadState , use ratatui :: layout :: { Alignment , Constraint , Direction , Layout , Rect } , use ratatui :: style :: { Color , Modifier , Style } , use ratatui :: text :: { Line , Span } , use ratatui :: widgets :: { Block , BorderType , Borders , Paragraph , Wrap } , use ratatui :: Frame 
- **Public Functions & Signatures**:
  ```rust
  fn draw (frame : & mut Frame , app : & App)
  ```

### `src/ui.rs` (Role: tui, Lines: 1)
- **Responsibility**: Core tui logic in src/ui.rs

## 4. Execution Lifecycle Trace
1. **Startup**: Entrypoint parses CLI flags & dispatches command.
2. **Execution**: Core domain logic processes inputs and evaluates rules.
3. **Persistence / I/O**: Domain logic calls infra for disk/terminal I/O.
4. **Exit**: Graceful termination with standard exit codes.

## 5. Verification Commands
```bash
cargo build --release --target x86_64-unknown-linux-gnu
cargo test --all-targets
cargo clippy --all-targets -- -D warnings && cargo fmt --check
```
