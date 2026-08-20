#![forbid(unsafe_code)]

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HarnessEventKind {
    BootstrapContext,
    UserTurn,
    FileRead,
    FileEdit,
    Validation,
    Checkpoint,
    Rehydrate,
    ContextDelta,
    LatticeOperation,
}

impl HarnessEventKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::BootstrapContext => "bootstrap_context",
            Self::UserTurn => "user_turn",
            Self::FileRead => "file_read",
            Self::FileEdit => "file_edit",
            Self::Validation => "validation",
            Self::Checkpoint => "checkpoint",
            Self::Rehydrate => "rehydrate",
            Self::ContextDelta => "context_delta",
            Self::LatticeOperation => "lattice_operation",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HarnessEvent {
    pub id: &'static str,
    pub kind: HarnessEventKind,
    pub summary: &'static str,
    pub source_pointer: Option<&'static str>,
    pub receipt: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HarnessReplay {
    pub fixture: &'static str,
    pub substrate: &'static str,
    pub active_cut: &'static str,
    pub checkpoint: &'static str,
    pub rehydrated_cut: &'static str,
    pub frontier_count: usize,
    pub events: Vec<HarnessEvent>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResponseDeltaKind {
    KeepIntent,
    AddSource,
    NarrowContext,
    JoinContext,
    FrontierGap,
    ValidationOwed,
    Checkpoint,
}

impl ResponseDeltaKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::KeepIntent => "keep_intent",
            Self::AddSource => "add_source",
            Self::NarrowContext => "narrow_context",
            Self::JoinContext => "join_context",
            Self::FrontierGap => "frontier_gap",
            Self::ValidationOwed => "validation_owed",
            Self::Checkpoint => "checkpoint",
        }
    }

    pub fn folded_operation(self) -> &'static str {
        match self {
            Self::KeepIntent => "preserve-active-cut",
            Self::AddSource => "source-pointer-candidate",
            Self::NarrowContext => "meet-with-constraint",
            Self::JoinContext => "join-with-source-cut",
            Self::FrontierGap => "frontier-unresolved-context",
            Self::ValidationOwed => "validation-receipt-required",
            Self::Checkpoint => "checkpoint-active-cut",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResponseDelta {
    pub id: &'static str,
    pub kind: ResponseDeltaKind,
    pub expression: &'static str,
    pub summary: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResponseDeltaPlan {
    pub fixture: &'static str,
    pub source_response: &'static str,
    pub active_cut: &'static str,
    pub folded_cut: &'static str,
    pub deltas: Vec<ResponseDelta>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeltaCoverageLevel {
    L0,
    L1,
    L2,
}

impl DeltaCoverageLevel {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::L0 => "L0",
            Self::L1 => "L1",
            Self::L2 => "L2",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeltaCoverageRow {
    pub level: DeltaCoverageLevel,
    pub name: &'static str,
    pub contract: &'static str,
    pub evidence_count: usize,
    pub complete: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeltaCoverageReport {
    pub fixture: &'static str,
    pub engine: &'static str,
    pub levels: Vec<DeltaCoverageRow>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InteractionScenario {
    pub id: String,
    pub family: &'static str,
    pub title: String,
    pub user_intent: String,
    pub expected_delta: ResponseDeltaKind,
    pub required_gate: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InteractionScenarioReport {
    pub suite: &'static str,
    pub provider_adapter: String,
    pub base_llm_decision_maker: String,
    pub provider_execution: &'static str,
    pub decision_boundary: &'static str,
    pub scenario_count: usize,
    pub family_count: usize,
    pub l0_covered_count: usize,
    pub l1_covered_count: usize,
    pub l2_covered_count: usize,
    pub permission_gate_count: usize,
    pub source_grounded_count: usize,
    pub checkpoint_count: usize,
    pub scenarios: Vec<InteractionScenario>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LatticeHandoffReport {
    pub fixture: &'static str,
    pub language_owner: &'static str,
    pub boundary: &'static str,
    pub lattice_replay_command: &'static str,
    pub lattice_replay_schema: &'static str,
    pub lattice_delta_schema: &'static str,
    pub lattice_pack_id: &'static str,
    pub lattice_prompt_frame_version: &'static str,
    pub lattice_frontier_status: &'static str,
    pub lattice_frontier_count: usize,
    pub lattice_receipt_citation: &'static str,
    pub source_pointers: Vec<LatticeSourcePointer>,
    pub cuts: Vec<LatticeCut>,
    pub receipts: Vec<LatticeReceipt>,
    pub frontiers: Vec<LatticeFrontier>,
    pub operation_intents: Vec<LatticeOperationIntent>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LatticeSourcePointer {
    pub id: &'static str,
    pub target: &'static str,
    pub evidence_role: &'static str,
    pub custody: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LatticeCut {
    pub id: &'static str,
    pub kind: &'static str,
    pub source: &'static str,
    pub receipt: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LatticeReceipt {
    pub id: &'static str,
    pub event: &'static str,
    pub owed: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LatticeFrontier {
    pub id: &'static str,
    pub reason: &'static str,
    pub next_gate: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LatticeOperationIntent {
    pub id: &'static str,
    pub operation: &'static str,
    pub source_delta: &'static str,
    pub status: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LatticeSourcePilotReport {
    pub schema: &'static str,
    pub consumed_command: &'static str,
    pub semantic_owner: &'static str,
    pub viewer_role: &'static str,
    pub source_id: &'static str,
    pub owner_repo: &'static str,
    pub work_id: &'static str,
    pub fletch_registry_path: &'static str,
    pub fletch_id: &'static str,
    pub proof_ledger_path: &'static str,
    pub proof_record_path: &'static str,
    pub rights_policy: &'static str,
    pub custody_receipt: &'static str,
    pub metadata_only: bool,
    pub source_content_copied: bool,
    pub display_lens: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MechanicalSandboxReport {
    pub schema: &'static str,
    pub fixture: &'static str,
    pub provider_execution: &'static str,
    pub preset_dataset: &'static str,
    pub user_condition: &'static str,
    pub consumed_lattice_schema: &'static str,
    pub lattice_delta_expression: &'static str,
    pub folded_operation: &'static str,
    pub reviewed_turn_boundary: &'static str,
    pub receipt_citation: &'static str,
    pub accepted_delta_count: usize,
    pub deferred_delta_count: usize,
    pub handoff_ready: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReviewerWorkflowReport {
    pub schema: &'static str,
    pub default_route: &'static str,
    pub semantic_owner: &'static str,
    pub viewer_role: &'static str,
    pub default_graph_view: &'static str,
    pub default_review_path: Vec<&'static str>,
    pub receipt_required: bool,
    pub provider_execution: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PressPreviewReport {
    pub schema: &'static str,
    pub consumed_command: &'static str,
    pub consumed_contract_schema: &'static str,
    pub semantic_owner: &'static str,
    pub preview_owner: &'static str,
    pub renderer_owner: &'static str,
    pub press_frame_id: &'static str,
    pub preview_scope: &'static str,
    pub preview_formats: Vec<&'static str>,
    pub witness_renders_publication: bool,
    pub lattice_renders_publication: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LatticeShardDisplayReport {
    pub schema: &'static str,
    pub consumed_command: &'static str,
    pub consumed_contract_schema: &'static str,
    pub semantic_owner: &'static str,
    pub viewer_role: &'static str,
    pub boundary: &'static str,
    pub aggregate_hash: &'static str,
    pub shard_count: usize,
    pub boundary_edge_count: usize,
    pub alignment_row_count: usize,
    pub frontier_count: usize,
    pub conflicting_boundary_count: usize,
    pub deterministic_replay: bool,
    pub witness_recomputes_alignment: bool,
    pub display_modes: Vec<&'static str>,
    pub shards: Vec<LatticeShardDisplayShard>,
    pub boundary_edges: Vec<LatticeShardDisplayEdge>,
    pub alignment_rows: Vec<LatticeShardDisplayRow>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LatticeShardDisplayShard {
    pub shard_id: &'static str,
    pub source_scope: &'static str,
    pub rights_policy: &'static str,
    pub status: &'static str,
    pub grain_count: usize,
    pub bond_count: usize,
    pub receipt_count: usize,
    pub frontier_count: usize,
    pub manifest_hash: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LatticeShardDisplayEdge {
    pub edge_id: &'static str,
    pub from_shard: &'static str,
    pub to_shard: &'static str,
    pub kind: &'static str,
    pub evidence_receipt: &'static str,
    pub rights_compatible: bool,
    pub alignment_status: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LatticeShardDisplayRow {
    pub row_id: &'static str,
    pub passed: bool,
    pub evidence: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LatticeShardScenarioDisplayReport {
    pub schema: &'static str,
    pub consumed_command: &'static str,
    pub consumed_contract_schema: &'static str,
    pub semantic_owner: &'static str,
    pub viewer_role: &'static str,
    pub boundary: &'static str,
    pub scenario_count: usize,
    pub scenario_hash: &'static str,
    pub witness_recomputes_routing: bool,
    pub examples: Vec<LatticeShardScenarioDisplayExample>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LatticeShardScenarioDisplayExample {
    pub scenario_id: &'static str,
    pub title: &'static str,
    pub selected_shards: Vec<&'static str>,
    pub frontier_shards: Vec<&'static str>,
    pub boundary_edges: Vec<&'static str>,
    pub expected_outcome: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LatticeShardValidationDisplayReport {
    pub schema: &'static str,
    pub consumed_command: &'static str,
    pub consumed_contract_schema: &'static str,
    pub semantic_owner: &'static str,
    pub viewer_role: &'static str,
    pub boundary: &'static str,
    pub aggregate_hash: &'static str,
    pub manifest_path: &'static str,
    pub receipt_path: &'static str,
    pub check_count: usize,
    pub validation_passed: bool,
    pub witness_recomputes_validation: bool,
    pub checks: Vec<LatticeShardValidationDisplayCheck>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LatticeShardValidationDisplayCheck {
    pub name: &'static str,
    pub passed: bool,
    pub note: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LatticeAlgebraValidationReport {
    pub suite: &'static str,
    pub consumed_command: &'static str,
    pub consumed_contract_schema: &'static str,
    pub semantic_owner: &'static str,
    pub viewer_role: &'static str,
    pub boundary: &'static str,
    pub passed: bool,
    pub rows: Vec<LatticeAlgebraPropertyRow>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LatticeAlgebraPropertyRow {
    pub id: &'static str,
    pub law: &'static str,
    pub operator: &'static str,
    pub left_grain_count: usize,
    pub right_grain_count: usize,
    pub passed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FormatMatrixReport {
    pub suite: &'static str,
    pub purpose: &'static str,
    pub rows: Vec<FormatMatrixRow>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FormatMatrixRow {
    pub target: &'static str,
    pub customer_need: &'static str,
    pub native_affordance: &'static str,
    pub witness_model: &'static str,
    pub lattice_validation: &'static str,
    pub projection_fidelity: u8,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckpointArtifact {
    pub fixture: &'static str,
    pub checkpoint_id: &'static str,
    pub active_cut: &'static str,
    pub rehydrated_cut: &'static str,
    pub receipt_count: usize,
    pub frontier_count: usize,
    pub source_pointer_count: usize,
    pub custody: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckpointWriteReport {
    pub artifact: CheckpointArtifact,
    pub path: String,
    pub byte_count: usize,
    pub checksum: u64,
    pub rehydration_status: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SessionCorpusReviewReport {
    pub suite: &'static str,
    pub purpose: &'static str,
    pub raw_ingestion_allowed: bool,
    pub gate_count: usize,
    pub approved_gate_count: usize,
    pub gates: Vec<SessionCorpusGate>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SessionCorpusGate {
    pub id: &'static str,
    pub requirement: &'static str,
    pub status: &'static str,
    pub lattice_validation: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MaximScenarioReport {
    pub suite: &'static str,
    pub corpus: &'static str,
    pub custody: &'static str,
    pub scenario_count: usize,
    pub pointer_count: usize,
    pub receipt_count: usize,
    pub frontier_count: usize,
    pub scenarios: Vec<MaximScenario>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MaximScenario {
    pub id: &'static str,
    pub user_goal: &'static str,
    pub source_pointer: &'static str,
    pub receipt: &'static str,
    pub lattice_validation: &'static str,
    pub result_shape: &'static str,
    pub saved_chunk: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MaximConversationReport {
    pub suite: &'static str,
    pub corpus: &'static str,
    pub provider_mode: &'static str,
    pub custody: &'static str,
    pub semantic_owner: &'static str,
    pub conversation_count: usize,
    pub turn_count: usize,
    pub pointer_count: usize,
    pub receipt_count: usize,
    pub visual_update_count: usize,
    pub passed_gate_count: usize,
    pub conversations: Vec<MaximConversation>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MaximConversation {
    pub id: &'static str,
    pub title: &'static str,
    pub user_goal: &'static str,
    pub source_pointer: &'static str,
    pub receipt: &'static str,
    pub expected_shape: &'static str,
    pub evaluation_gate: &'static str,
    pub turns: Vec<MaximConversationTurn>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MaximConversationTurn {
    pub id: &'static str,
    pub speaker: &'static str,
    pub summary: &'static str,
    pub harness_event: &'static str,
    pub visual_update: &'static str,
    pub lattice_anchor: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MaximConversationEvaluationReport {
    pub suite: &'static str,
    pub corpus: &'static str,
    pub evaluation_target: &'static str,
    pub semantic_owner: &'static str,
    pub scenario_count: usize,
    pub passed_count: usize,
    pub failed_count: usize,
    pub gate_count: usize,
    pub results: Vec<MaximConversationEvaluation>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MaximConversationEvaluation {
    pub id: &'static str,
    pub title: &'static str,
    pub pointer_gate: &'static str,
    pub receipt_gate: &'static str,
    pub visual_gate: &'static str,
    pub frontier_gate: &'static str,
    pub shape_gate: &'static str,
    pub result: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MaximConversationReplayReport {
    pub suite: &'static str,
    pub transcript: &'static str,
    pub corpus: &'static str,
    pub semantic_owner: &'static str,
    pub frame_count: usize,
    pub visual_frame_count: usize,
    pub source_anchor_count: usize,
    pub frontier_frame_count: usize,
    pub frames: Vec<MaximConversationReplayFrame>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MaximConversationReplayFrame {
    pub turn_id: &'static str,
    pub speaker: &'static str,
    pub message: &'static str,
    pub expected_delta: &'static str,
    pub visual_frame: &'static str,
    pub lattice_anchor: &'static str,
    pub assertion: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MaximConversationSyncReport {
    pub suite: &'static str,
    pub transcript: &'static str,
    pub sync_target: &'static str,
    pub semantic_owner: &'static str,
    pub check_count: usize,
    pub passed_count: usize,
    pub failed_count: usize,
    pub checks: Vec<MaximConversationSyncCheck>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MaximConversationSyncCheck {
    pub id: &'static str,
    pub replay_frame: &'static str,
    pub chat_surface_state: &'static str,
    pub visual_contract: &'static str,
    pub expected_artifact: &'static str,
    pub sync_gate: &'static str,
    pub result: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChatWebReadinessReport {
    pub suite: &'static str,
    pub intended_command: &'static str,
    pub host_policy: &'static str,
    pub semantic_owner: &'static str,
    pub route_count: usize,
    pub data_feed_count: usize,
    pub privacy_gate_count: usize,
    pub ready_gate_count: usize,
    pub routes: Vec<ChatWebRoute>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChatWebRoute {
    pub path: &'static str,
    pub surface: &'static str,
    pub data_feed: &'static str,
    pub render_state: &'static str,
    pub privacy_gate: &'static str,
    pub ready_gate: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChatWebFixtureBundleReport {
    pub suite: &'static str,
    pub bootstrap_route: &'static str,
    pub host_policy: &'static str,
    pub semantic_owner: &'static str,
    pub feed_count: usize,
    pub privacy_gate: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChatWebStoryboardReport {
    pub suite: &'static str,
    pub target_surface: &'static str,
    pub semantic_owner: &'static str,
    pub panel_count: usize,
    pub frame_count: usize,
    pub panels: Vec<ChatWebPanel>,
    pub frames: Vec<ChatWebStoryboardFrame>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChatWebPanel {
    pub id: &'static str,
    pub title: &'static str,
    pub role: &'static str,
    pub data_feed: &'static str,
    pub render_contract: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChatWebStoryboardFrame {
    pub id: &'static str,
    pub replay_frame: &'static str,
    pub active_panel: &'static str,
    pub animation_cue: &'static str,
    pub user_feedback: &'static str,
    pub frontier_state: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChatWebAcceptanceReport {
    pub suite: &'static str,
    pub target_command: &'static str,
    pub semantic_owner: &'static str,
    pub criterion_count: usize,
    pub required_count: usize,
    pub criteria: Vec<ChatWebAcceptanceCriterion>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChatWebAcceptanceCriterion {
    pub id: &'static str,
    pub requirement: &'static str,
    pub evidence_feed: &'static str,
    pub expected_check: &'static str,
    pub gate: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VisualContextReport {
    pub suite: &'static str,
    pub scenario: &'static str,
    pub goal_artifact: &'static str,
    pub semantic_owner: &'static str,
    pub event_count: usize,
    pub saved_chunk_count: usize,
    pub frontier_count: usize,
    pub events: Vec<VisualContextEvent>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VisualContextEvent {
    pub id: &'static str,
    pub motion: &'static str,
    pub lattice_contract: &'static str,
    pub user_feedback: &'static str,
    pub shape: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VisualShapeCatalog {
    pub suite: &'static str,
    pub semantic_owner: &'static str,
    pub shape_count: usize,
    pub shapes: Vec<VisualShape>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VisualShape {
    pub id: &'static str,
    pub scenario: &'static str,
    pub artifact_shape: &'static str,
    pub user_satisfaction: &'static str,
    pub required_lattice_evidence: &'static str,
    pub saveable_chunk: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChunkInventoryReport {
    pub suite: &'static str,
    pub semantic_owner: &'static str,
    pub inventory_status: &'static str,
    pub chunk_count: usize,
    pub reusable_count: usize,
    pub frontier_count: usize,
    pub chunks: Vec<ContextChunk>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContextChunk {
    pub id: &'static str,
    pub title: &'static str,
    pub source: &'static str,
    pub artifact_shape: &'static str,
    pub lattice_anchor: &'static str,
    pub custody: &'static str,
    pub reuse_status: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BuildMeterReport {
    pub suite: &'static str,
    pub scenario: &'static str,
    pub semantic_owner: &'static str,
    pub overall_progress: u8,
    pub stage_count: usize,
    pub stages: Vec<BuildMeterStage>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BuildMeterStage {
    pub id: &'static str,
    pub label: &'static str,
    pub progress: u8,
    pub signal: &'static str,
    pub lattice_anchor: &'static str,
    pub visual_feedback: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArtifactAssemblyReport {
    pub suite: &'static str,
    pub scenario: &'static str,
    pub artifact_shape: &'static str,
    pub semantic_owner: &'static str,
    pub piece_count: usize,
    pub included_count: usize,
    pub frontier_count: usize,
    pub pieces: Vec<ArtifactPiece>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArtifactPiece {
    pub id: &'static str,
    pub label: &'static str,
    pub role: &'static str,
    pub lattice_anchor: &'static str,
    pub fit_status: &'static str,
    pub visual_feedback: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NarrowingTraceReport {
    pub suite: &'static str,
    pub scenario: &'static str,
    pub semantic_owner: &'static str,
    pub initial_candidate_count: usize,
    pub final_candidate_count: usize,
    pub saved_chunk_count: usize,
    pub frontier_count: usize,
    pub steps: Vec<NarrowingStep>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NarrowingStep {
    pub id: &'static str,
    pub query: &'static str,
    pub lattice_operation: &'static str,
    pub before_count: usize,
    pub after_count: usize,
    pub saved_chunk: &'static str,
    pub frontier: &'static str,
    pub visual_feedback: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConversationSurfaceReport {
    pub suite: &'static str,
    pub scenario: &'static str,
    pub primary_surface: &'static str,
    pub companion_surface: &'static str,
    pub semantic_owner: &'static str,
    pub turn_count: usize,
    pub visual_update_count: usize,
    pub frontier_count: usize,
    pub turns: Vec<ConversationTurn>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConversationTurn {
    pub id: &'static str,
    pub speaker: &'static str,
    pub message_summary: &'static str,
    pub harness_event: &'static str,
    pub visual_state: &'static str,
    pub lattice_anchor: &'static str,
    pub frontier: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OperatorShowcaseReport {
    pub suite: &'static str,
    pub semantic_owner: &'static str,
    pub boundary: &'static str,
    pub generic_dialect_system: bool,
    pub showcases: Vec<OperatorShowcase>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LatticeScenarioOperatorCatalogReport {
    pub suite: &'static str,
    pub consumed_contract_schema: &'static str,
    pub semantic_owner: &'static str,
    pub edge_viewer: &'static str,
    pub boundary: &'static str,
    pub generic_dialect_system: bool,
    pub scenarios: Vec<LatticeScenarioOperatorChain>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LatticeScenarioOperatorChain {
    pub id: &'static str,
    pub label: &'static str,
    pub operators: &'static [&'static str],
    pub summary: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OperatorShowcase {
    pub id: &'static str,
    pub label: &'static str,
    pub scenario: &'static str,
    pub input_layer: &'static str,
    pub operator_role: &'static str,
    pub output_layer: &'static str,
    pub dataflow: &'static str,
    pub edge_note: &'static str,
}

impl OperatorShowcaseReport {
    pub fn operator_count(&self) -> usize {
        self.showcases.len()
    }

    pub fn to_json(&self) -> String {
        let showcases = self
            .showcases
            .iter()
            .map(OperatorShowcase::to_json)
            .collect::<Vec<_>>()
            .join(",");
        format!(
            concat!(
                "{{",
                "\"schema\":\"witness.operator-showcases.v1\",",
                "\"suite\":\"{}\",",
                "\"semantic_owner\":\"{}\",",
                "\"boundary\":\"{}\",",
                "\"generic_dialect_system\":{},",
                "\"operator_count\":{},",
                "\"showcases\":[{}]",
                "}}"
            ),
            escape_json(self.suite),
            escape_json(self.semantic_owner),
            escape_json(self.boundary),
            self.generic_dialect_system,
            self.operator_count(),
            showcases
        )
    }
}

impl LatticeScenarioOperatorCatalogReport {
    pub fn scenario_count(&self) -> usize {
        self.scenarios.len()
    }

    pub fn to_json(&self) -> String {
        let scenarios = self
            .scenarios
            .iter()
            .map(LatticeScenarioOperatorChain::to_json)
            .collect::<Vec<_>>()
            .join(",");
        format!(
            concat!(
                "{{",
                "\"schema\":\"witness.lattice-scenario-operators.v1\",",
                "\"suite\":\"{}\",",
                "\"consumed_contract_schema\":\"{}\",",
                "\"semantic_owner\":\"{}\",",
                "\"edge_viewer\":\"{}\",",
                "\"boundary\":\"{}\",",
                "\"generic_dialect_system\":{},",
                "\"scenario_count\":{},",
                "\"scenarios\":[{}]",
                "}}"
            ),
            escape_json(self.suite),
            escape_json(self.consumed_contract_schema),
            escape_json(self.semantic_owner),
            escape_json(self.edge_viewer),
            escape_json(self.boundary),
            self.generic_dialect_system,
            self.scenario_count(),
            scenarios
        )
    }
}

impl LatticeScenarioOperatorChain {
    fn to_json(&self) -> String {
        let operators = self
            .operators
            .iter()
            .map(|operator| format!("\"{}\"", escape_json(operator)))
            .collect::<Vec<_>>()
            .join(",");
        format!(
            concat!(
                "{{",
                "\"id\":\"{}\",",
                "\"label\":\"{}\",",
                "\"operators\":[{}],",
                "\"summary\":\"{}\"",
                "}}"
            ),
            escape_json(self.id),
            escape_json(self.label),
            operators,
            escape_json(self.summary)
        )
    }
}

impl OperatorShowcase {
    fn to_json(&self) -> String {
        format!(
            concat!(
                "{{",
                "\"id\":\"{}\",",
                "\"label\":\"{}\",",
                "\"scenario\":\"{}\",",
                "\"input_layer\":\"{}\",",
                "\"operator_role\":\"{}\",",
                "\"output_layer\":\"{}\",",
                "\"dataflow\":\"{}\",",
                "\"edge_note\":\"{}\"",
                "}}"
            ),
            escape_json(self.id),
            escape_json(self.label),
            escape_json(self.scenario),
            escape_json(self.input_layer),
            escape_json(self.operator_role),
            escape_json(self.output_layer),
            escape_json(self.dataflow),
            escape_json(self.edge_note)
        )
    }
}

impl DeltaCoverageReport {
    pub fn level_count(&self) -> usize {
        self.levels.len()
    }

    pub fn complete_count(&self) -> usize {
        self.levels.iter().filter(|level| level.complete).count()
    }

    pub fn total_evidence_count(&self) -> usize {
        self.levels.iter().map(|level| level.evidence_count).sum()
    }

    pub fn to_json(&self) -> String {
        let levels = self
            .levels
            .iter()
            .map(DeltaCoverageRow::to_json)
            .collect::<Vec<_>>()
            .join(",");
        format!(
            concat!(
                "{{",
                "\"schema\":\"witness.delta-coverage.v1\",",
                "\"fixture\":\"{}\",",
                "\"engine\":\"{}\",",
                "\"level_count\":{},",
                "\"complete_count\":{},",
                "\"total_evidence_count\":{},",
                "\"levels\":[{}]",
                "}}"
            ),
            escape_json(self.fixture),
            escape_json(self.engine),
            self.level_count(),
            self.complete_count(),
            self.total_evidence_count(),
            levels
        )
    }
}

impl DeltaCoverageRow {
    fn to_json(&self) -> String {
        format!(
            concat!(
                "{{",
                "\"level\":\"{}\",",
                "\"name\":\"{}\",",
                "\"contract\":\"{}\",",
                "\"evidence_count\":{},",
                "\"complete\":{}",
                "}}"
            ),
            self.level.as_str(),
            escape_json(self.name),
            escape_json(self.contract),
            self.evidence_count,
            self.complete
        )
    }
}

impl InteractionScenarioReport {
    pub fn to_json(&self) -> String {
        let scenarios = self
            .scenarios
            .iter()
            .map(InteractionScenario::to_json)
            .collect::<Vec<_>>()
            .join(",");
        format!(
            concat!(
                "{{",
                "\"schema\":\"witness.interaction-scenarios.v1\",",
                "\"suite\":\"{}\",",
                "\"provider_adapter\":\"{}\",",
                "\"base_llm_decision_maker\":\"{}\",",
                "\"provider_execution\":\"{}\",",
                "\"decision_boundary\":\"{}\",",
                "\"scenario_count\":{},",
                "\"family_count\":{},",
                "\"l0_covered_count\":{},",
                "\"l1_covered_count\":{},",
                "\"l2_covered_count\":{},",
                "\"permission_gate_count\":{},",
                "\"source_grounded_count\":{},",
                "\"checkpoint_count\":{},",
                "\"scenarios\":[{}]",
                "}}"
            ),
            escape_json(self.suite),
            escape_json(&self.provider_adapter),
            escape_json(&self.base_llm_decision_maker),
            self.provider_execution,
            escape_json(self.decision_boundary),
            self.scenario_count,
            self.family_count,
            self.l0_covered_count,
            self.l1_covered_count,
            self.l2_covered_count,
            self.permission_gate_count,
            self.source_grounded_count,
            self.checkpoint_count,
            scenarios
        )
    }
}

impl InteractionScenario {
    fn to_json(&self) -> String {
        format!(
            concat!(
                "{{",
                "\"id\":\"{}\",",
                "\"family\":\"{}\",",
                "\"title\":\"{}\",",
                "\"user_intent\":\"{}\",",
                "\"expected_delta\":\"{}\",",
                "\"folded_operation\":\"{}\",",
                "\"required_gate\":\"{}\"",
                "}}"
            ),
            escape_json(&self.id),
            self.family,
            escape_json(&self.title),
            escape_json(&self.user_intent),
            self.expected_delta.as_str(),
            self.expected_delta.folded_operation(),
            self.required_gate
        )
    }
}

impl LatticeHandoffReport {
    pub fn source_pointer_count(&self) -> usize {
        self.source_pointers.len()
    }

    pub fn cut_count(&self) -> usize {
        self.cuts.len()
    }

    pub fn receipt_count(&self) -> usize {
        self.receipts.len()
    }

    pub fn frontier_count(&self) -> usize {
        self.frontiers.len()
    }

    pub fn operation_intent_count(&self) -> usize {
        self.operation_intents.len()
    }

    pub fn to_json(&self) -> String {
        let source_pointers = self
            .source_pointers
            .iter()
            .map(LatticeSourcePointer::to_json)
            .collect::<Vec<_>>()
            .join(",");
        let cuts = self
            .cuts
            .iter()
            .map(LatticeCut::to_json)
            .collect::<Vec<_>>()
            .join(",");
        let receipts = self
            .receipts
            .iter()
            .map(LatticeReceipt::to_json)
            .collect::<Vec<_>>()
            .join(",");
        let frontiers = self
            .frontiers
            .iter()
            .map(LatticeFrontier::to_json)
            .collect::<Vec<_>>()
            .join(",");
        let operation_intents = self
            .operation_intents
            .iter()
            .map(LatticeOperationIntent::to_json)
            .collect::<Vec<_>>()
            .join(",");
        format!(
            concat!(
                "{{",
                "\"schema\":\"witness.lattice-handoff.v1\",",
                "\"fixture\":\"{}\",",
                "\"language_owner\":\"{}\",",
                "\"boundary\":\"{}\",",
                "\"lattice_replay_command\":\"{}\",",
                "\"lattice_replay_schema\":\"{}\",",
                "\"lattice_delta_schema\":\"{}\",",
                "\"lattice_pack_id\":\"{}\",",
                "\"lattice_prompt_frame_version\":\"{}\",",
                "\"lattice_frontier_status\":\"{}\",",
                "\"lattice_frontier_count\":{},",
                "\"lattice_receipt_citation\":\"{}\",",
                "\"source_pointer_count\":{},",
                "\"cut_count\":{},",
                "\"receipt_count\":{},",
                "\"frontier_count\":{},",
                "\"operation_intent_count\":{},",
                "\"source_pointers\":[{}],",
                "\"cuts\":[{}],",
                "\"receipts\":[{}],",
                "\"frontiers\":[{}],",
                "\"operation_intents\":[{}]",
                "}}"
            ),
            escape_json(self.fixture),
            self.language_owner,
            escape_json(self.boundary),
            escape_json(self.lattice_replay_command),
            escape_json(self.lattice_replay_schema),
            escape_json(self.lattice_delta_schema),
            escape_json(self.lattice_pack_id),
            escape_json(self.lattice_prompt_frame_version),
            escape_json(self.lattice_frontier_status),
            self.lattice_frontier_count,
            escape_json(self.lattice_receipt_citation),
            self.source_pointer_count(),
            self.cut_count(),
            self.receipt_count(),
            self.frontier_count(),
            self.operation_intent_count(),
            source_pointers,
            cuts,
            receipts,
            frontiers,
            operation_intents
        )
    }
}

impl LatticeSourcePointer {
    fn to_json(&self) -> String {
        format!(
            concat!(
                "{{",
                "\"id\":\"{}\",",
                "\"target\":\"{}\",",
                "\"evidence_role\":\"{}\",",
                "\"custody\":\"{}\"",
                "}}"
            ),
            escape_json(self.id),
            escape_json(self.target),
            escape_json(self.evidence_role),
            escape_json(self.custody)
        )
    }
}

impl LatticeSourcePilotReport {
    pub fn to_json(&self) -> String {
        format!(
            concat!(
                "{{",
                "\"schema\":\"{}\",",
                "\"consumed_command\":\"{}\",",
                "\"semantic_owner\":\"{}\",",
                "\"viewer_role\":\"{}\",",
                "\"source_id\":\"{}\",",
                "\"owner_repo\":\"{}\",",
                "\"work_id\":\"{}\",",
                "\"fletch_registry_path\":\"{}\",",
                "\"fletch_id\":\"{}\",",
                "\"proof_ledger_path\":\"{}\",",
                "\"proof_record_path\":\"{}\",",
                "\"rights_policy\":\"{}\",",
                "\"custody_receipt\":\"{}\",",
                "\"metadata_only\":{},",
                "\"source_content_copied\":{},",
                "\"display_lens\":\"{}\"",
                "}}"
            ),
            escape_json(self.schema),
            escape_json(self.consumed_command),
            escape_json(self.semantic_owner),
            escape_json(self.viewer_role),
            escape_json(self.source_id),
            escape_json(self.owner_repo),
            escape_json(self.work_id),
            escape_json(self.fletch_registry_path),
            escape_json(self.fletch_id),
            escape_json(self.proof_ledger_path),
            escape_json(self.proof_record_path),
            escape_json(self.rights_policy),
            escape_json(self.custody_receipt),
            self.metadata_only,
            self.source_content_copied,
            escape_json(self.display_lens)
        )
    }
}

impl MechanicalSandboxReport {
    pub fn to_json(&self) -> String {
        format!(
            concat!(
                "{{",
                "\"schema\":\"{}\",",
                "\"fixture\":\"{}\",",
                "\"provider_execution\":\"{}\",",
                "\"preset_dataset\":\"{}\",",
                "\"user_condition\":\"{}\",",
                "\"consumed_lattice_schema\":\"{}\",",
                "\"lattice_delta_expression\":\"{}\",",
                "\"folded_operation\":\"{}\",",
                "\"reviewed_turn_boundary\":\"{}\",",
                "\"receipt_citation\":\"{}\",",
                "\"accepted_delta_count\":{},",
                "\"deferred_delta_count\":{},",
                "\"handoff_ready\":{}",
                "}}"
            ),
            escape_json(self.schema),
            escape_json(self.fixture),
            escape_json(self.provider_execution),
            escape_json(self.preset_dataset),
            escape_json(self.user_condition),
            escape_json(self.consumed_lattice_schema),
            escape_json(self.lattice_delta_expression),
            escape_json(self.folded_operation),
            escape_json(self.reviewed_turn_boundary),
            escape_json(self.receipt_citation),
            self.accepted_delta_count,
            self.deferred_delta_count,
            self.handoff_ready
        )
    }
}

impl ReviewerWorkflowReport {
    pub fn to_json(&self) -> String {
        format!(
            concat!(
                "{{",
                "\"schema\":\"{}\",",
                "\"default_route\":\"{}\",",
                "\"semantic_owner\":\"{}\",",
                "\"viewer_role\":\"{}\",",
                "\"default_graph_view\":\"{}\",",
                "\"default_review_path\":[{}],",
                "\"receipt_required\":{},",
                "\"provider_execution\":\"{}\"",
                "}}"
            ),
            escape_json(self.schema),
            escape_json(self.default_route),
            escape_json(self.semantic_owner),
            escape_json(self.viewer_role),
            escape_json(self.default_graph_view),
            json_string_array(&self.default_review_path),
            self.receipt_required,
            escape_json(self.provider_execution)
        )
    }
}

impl PressPreviewReport {
    pub fn to_json(&self) -> String {
        format!(
            concat!(
                "{{",
                "\"schema\":\"{}\",",
                "\"consumed_command\":\"{}\",",
                "\"consumed_contract_schema\":\"{}\",",
                "\"semantic_owner\":\"{}\",",
                "\"preview_owner\":\"{}\",",
                "\"renderer_owner\":\"{}\",",
                "\"press_frame_id\":\"{}\",",
                "\"preview_scope\":\"{}\",",
                "\"preview_formats\":[{}],",
                "\"witness_renders_publication\":{},",
                "\"lattice_renders_publication\":{}",
                "}}"
            ),
            escape_json(self.schema),
            escape_json(self.consumed_command),
            escape_json(self.consumed_contract_schema),
            escape_json(self.semantic_owner),
            escape_json(self.preview_owner),
            escape_json(self.renderer_owner),
            escape_json(self.press_frame_id),
            escape_json(self.preview_scope),
            json_string_array(&self.preview_formats),
            self.witness_renders_publication,
            self.lattice_renders_publication
        )
    }
}

impl LatticeAlgebraValidationReport {
    pub fn property_count(&self) -> usize {
        self.rows.len()
    }

    pub fn to_json(&self) -> String {
        let rows = self
            .rows
            .iter()
            .map(LatticeAlgebraPropertyRow::to_json)
            .collect::<Vec<_>>()
            .join(",");
        format!(
            concat!(
                "{{",
                "\"schema\":\"witness.lattice-algebra-validation.v1\",",
                "\"suite\":\"{}\",",
                "\"consumed_command\":\"{}\",",
                "\"consumed_contract_schema\":\"{}\",",
                "\"semantic_owner\":\"{}\",",
                "\"viewer_role\":\"{}\",",
                "\"boundary\":\"{}\",",
                "\"property_count\":{},",
                "\"passed\":{},",
                "\"rows\":[{}]",
                "}}"
            ),
            escape_json(self.suite),
            escape_json(self.consumed_command),
            escape_json(self.consumed_contract_schema),
            escape_json(self.semantic_owner),
            escape_json(self.viewer_role),
            escape_json(self.boundary),
            self.property_count(),
            self.passed,
            rows
        )
    }
}

impl LatticeShardDisplayReport {
    pub fn to_json(&self) -> String {
        let display_modes = json_string_array(&self.display_modes);
        let shards = self
            .shards
            .iter()
            .map(LatticeShardDisplayShard::to_json)
            .collect::<Vec<_>>()
            .join(",");
        let boundary_edges = self
            .boundary_edges
            .iter()
            .map(LatticeShardDisplayEdge::to_json)
            .collect::<Vec<_>>()
            .join(",");
        let alignment_rows = self
            .alignment_rows
            .iter()
            .map(LatticeShardDisplayRow::to_json)
            .collect::<Vec<_>>()
            .join(",");
        format!(
            concat!(
                "{{",
                "\"schema\":\"{}\",",
                "\"consumed_command\":\"{}\",",
                "\"consumed_contract_schema\":\"{}\",",
                "\"semantic_owner\":\"{}\",",
                "\"viewer_role\":\"{}\",",
                "\"boundary\":\"{}\",",
                "\"aggregate_hash\":\"{}\",",
                "\"shard_count\":{},",
                "\"boundary_edge_count\":{},",
                "\"alignment_row_count\":{},",
                "\"frontier_count\":{},",
                "\"conflicting_boundary_count\":{},",
                "\"deterministic_replay\":{},",
                "\"witness_recomputes_alignment\":{},",
                "\"display_modes\":[{}],",
                "\"shards\":[{}],",
                "\"boundary_edges\":[{}],",
                "\"alignment_rows\":[{}]",
                "}}"
            ),
            escape_json(self.schema),
            escape_json(self.consumed_command),
            escape_json(self.consumed_contract_schema),
            escape_json(self.semantic_owner),
            escape_json(self.viewer_role),
            escape_json(self.boundary),
            escape_json(self.aggregate_hash),
            self.shard_count,
            self.boundary_edge_count,
            self.alignment_row_count,
            self.frontier_count,
            self.conflicting_boundary_count,
            self.deterministic_replay,
            self.witness_recomputes_alignment,
            display_modes,
            shards,
            boundary_edges,
            alignment_rows
        )
    }
}

impl LatticeShardDisplayShard {
    fn to_json(&self) -> String {
        format!(
            concat!(
                "{{",
                "\"shard_id\":\"{}\",",
                "\"source_scope\":\"{}\",",
                "\"rights_policy\":\"{}\",",
                "\"status\":\"{}\",",
                "\"grain_count\":{},",
                "\"bond_count\":{},",
                "\"receipt_count\":{},",
                "\"frontier_count\":{},",
                "\"manifest_hash\":\"{}\"",
                "}}"
            ),
            escape_json(self.shard_id),
            escape_json(self.source_scope),
            escape_json(self.rights_policy),
            escape_json(self.status),
            self.grain_count,
            self.bond_count,
            self.receipt_count,
            self.frontier_count,
            escape_json(self.manifest_hash)
        )
    }
}

impl LatticeShardDisplayEdge {
    fn to_json(&self) -> String {
        format!(
            concat!(
                "{{",
                "\"edge_id\":\"{}\",",
                "\"from_shard\":\"{}\",",
                "\"to_shard\":\"{}\",",
                "\"kind\":\"{}\",",
                "\"evidence_receipt\":\"{}\",",
                "\"rights_compatible\":{},",
                "\"alignment_status\":\"{}\"",
                "}}"
            ),
            escape_json(self.edge_id),
            escape_json(self.from_shard),
            escape_json(self.to_shard),
            escape_json(self.kind),
            escape_json(self.evidence_receipt),
            self.rights_compatible,
            escape_json(self.alignment_status)
        )
    }
}

impl LatticeShardDisplayRow {
    fn to_json(&self) -> String {
        format!(
            concat!(
                "{{",
                "\"row_id\":\"{}\",",
                "\"passed\":{},",
                "\"evidence\":\"{}\"",
                "}}"
            ),
            escape_json(self.row_id),
            self.passed,
            escape_json(self.evidence)
        )
    }
}

impl LatticeShardScenarioDisplayReport {
    pub fn to_json(&self) -> String {
        let examples = self
            .examples
            .iter()
            .map(LatticeShardScenarioDisplayExample::to_json)
            .collect::<Vec<_>>()
            .join(",");
        format!(
            concat!(
                "{{",
                "\"schema\":\"{}\",",
                "\"consumed_command\":\"{}\",",
                "\"consumed_contract_schema\":\"{}\",",
                "\"semantic_owner\":\"{}\",",
                "\"viewer_role\":\"{}\",",
                "\"boundary\":\"{}\",",
                "\"scenario_count\":{},",
                "\"scenario_hash\":\"{}\",",
                "\"witness_recomputes_routing\":{},",
                "\"examples\":[{}]",
                "}}"
            ),
            escape_json(self.schema),
            escape_json(self.consumed_command),
            escape_json(self.consumed_contract_schema),
            escape_json(self.semantic_owner),
            escape_json(self.viewer_role),
            escape_json(self.boundary),
            self.scenario_count,
            escape_json(self.scenario_hash),
            self.witness_recomputes_routing,
            examples
        )
    }
}

impl LatticeShardScenarioDisplayExample {
    fn to_json(&self) -> String {
        format!(
            concat!(
                "{{",
                "\"scenario_id\":\"{}\",",
                "\"title\":\"{}\",",
                "\"selected_shards\":[{}],",
                "\"frontier_shards\":[{}],",
                "\"boundary_edges\":[{}],",
                "\"expected_outcome\":\"{}\"",
                "}}"
            ),
            escape_json(self.scenario_id),
            escape_json(self.title),
            json_string_array(&self.selected_shards),
            json_string_array(&self.frontier_shards),
            json_string_array(&self.boundary_edges),
            escape_json(self.expected_outcome)
        )
    }
}

impl LatticeShardValidationDisplayReport {
    pub fn to_json(&self) -> String {
        let checks = self
            .checks
            .iter()
            .map(LatticeShardValidationDisplayCheck::to_json)
            .collect::<Vec<_>>()
            .join(",");
        format!(
            concat!(
                "{{",
                "\"schema\":\"{}\",",
                "\"consumed_command\":\"{}\",",
                "\"consumed_contract_schema\":\"{}\",",
                "\"semantic_owner\":\"{}\",",
                "\"viewer_role\":\"{}\",",
                "\"boundary\":\"{}\",",
                "\"aggregate_hash\":\"{}\",",
                "\"manifest_path\":\"{}\",",
                "\"receipt_path\":\"{}\",",
                "\"check_count\":{},",
                "\"validation_passed\":{},",
                "\"witness_recomputes_validation\":{},",
                "\"checks\":[{}]",
                "}}"
            ),
            escape_json(self.schema),
            escape_json(self.consumed_command),
            escape_json(self.consumed_contract_schema),
            escape_json(self.semantic_owner),
            escape_json(self.viewer_role),
            escape_json(self.boundary),
            escape_json(self.aggregate_hash),
            escape_json(self.manifest_path),
            escape_json(self.receipt_path),
            self.check_count,
            self.validation_passed,
            self.witness_recomputes_validation,
            checks
        )
    }
}

impl LatticeShardValidationDisplayCheck {
    fn to_json(&self) -> String {
        format!(
            concat!(
                "{{",
                "\"name\":\"{}\",",
                "\"passed\":{},",
                "\"note\":\"{}\"",
                "}}"
            ),
            escape_json(self.name),
            self.passed,
            escape_json(self.note)
        )
    }
}

impl LatticeAlgebraPropertyRow {
    fn to_json(&self) -> String {
        format!(
            concat!(
                "{{",
                "\"id\":\"{}\",",
                "\"law\":\"{}\",",
                "\"operator\":\"{}\",",
                "\"left_grain_count\":{},",
                "\"right_grain_count\":{},",
                "\"passed\":{}",
                "}}"
            ),
            escape_json(self.id),
            escape_json(self.law),
            escape_json(self.operator),
            self.left_grain_count,
            self.right_grain_count,
            self.passed
        )
    }
}

impl LatticeCut {
    fn to_json(&self) -> String {
        format!(
            concat!(
                "{{",
                "\"id\":\"{}\",",
                "\"kind\":\"{}\",",
                "\"source\":\"{}\",",
                "\"receipt\":\"{}\"",
                "}}"
            ),
            escape_json(self.id),
            self.kind,
            escape_json(self.source),
            escape_json(self.receipt)
        )
    }
}

impl LatticeReceipt {
    fn to_json(&self) -> String {
        format!(
            concat!(
                "{{",
                "\"id\":\"{}\",",
                "\"event\":\"{}\",",
                "\"owed\":\"{}\"",
                "}}"
            ),
            escape_json(self.id),
            escape_json(self.event),
            escape_json(self.owed)
        )
    }
}

impl LatticeFrontier {
    fn to_json(&self) -> String {
        format!(
            concat!(
                "{{",
                "\"id\":\"{}\",",
                "\"reason\":\"{}\",",
                "\"next_gate\":\"{}\"",
                "}}"
            ),
            escape_json(self.id),
            escape_json(self.reason),
            escape_json(self.next_gate)
        )
    }
}

impl LatticeOperationIntent {
    fn to_json(&self) -> String {
        format!(
            concat!(
                "{{",
                "\"id\":\"{}\",",
                "\"operation\":\"{}\",",
                "\"source_delta\":\"{}\",",
                "\"status\":\"{}\"",
                "}}"
            ),
            escape_json(self.id),
            escape_json(self.operation),
            escape_json(self.source_delta),
            escape_json(self.status)
        )
    }
}

impl FormatMatrixReport {
    pub fn target_count(&self) -> usize {
        self.rows.len()
    }

    pub fn average_projection_fidelity(&self) -> u8 {
        let total = self
            .rows
            .iter()
            .map(|row| usize::from(row.projection_fidelity))
            .sum::<usize>();
        (total / self.rows.len()) as u8
    }

    pub fn to_json(&self) -> String {
        let rows = self
            .rows
            .iter()
            .map(FormatMatrixRow::to_json)
            .collect::<Vec<_>>()
            .join(",");
        format!(
            concat!(
                "{{",
                "\"schema\":\"witness.format-matrix.v1\",",
                "\"suite\":\"{}\",",
                "\"purpose\":\"{}\",",
                "\"target_count\":{},",
                "\"average_projection_fidelity\":{},",
                "\"rows\":[{}]",
                "}}"
            ),
            escape_json(self.suite),
            escape_json(self.purpose),
            self.target_count(),
            self.average_projection_fidelity(),
            rows
        )
    }
}

impl FormatMatrixRow {
    fn to_json(&self) -> String {
        format!(
            concat!(
                "{{",
                "\"target\":\"{}\",",
                "\"customer_need\":\"{}\",",
                "\"native_affordance\":\"{}\",",
                "\"witness_model\":\"{}\",",
                "\"lattice_validation\":\"{}\",",
                "\"projection_fidelity\":{}",
                "}}"
            ),
            escape_json(self.target),
            escape_json(self.customer_need),
            escape_json(self.native_affordance),
            escape_json(self.witness_model),
            escape_json(self.lattice_validation),
            self.projection_fidelity
        )
    }
}

impl CheckpointArtifact {
    pub fn to_json(&self) -> String {
        format!(
            concat!(
                "{{",
                "\"schema\":\"witness.checkpoint-artifact.v1\",",
                "\"fixture\":\"{}\",",
                "\"checkpoint_id\":\"{}\",",
                "\"active_cut\":\"{}\",",
                "\"rehydrated_cut\":\"{}\",",
                "\"receipt_count\":{},",
                "\"frontier_count\":{},",
                "\"source_pointer_count\":{},",
                "\"custody\":\"{}\"",
                "}}"
            ),
            escape_json(self.fixture),
            escape_json(self.checkpoint_id),
            escape_json(self.active_cut),
            escape_json(self.rehydrated_cut),
            self.receipt_count,
            self.frontier_count,
            self.source_pointer_count,
            escape_json(self.custody)
        )
    }

    pub fn write_report(&self, path: &str) -> CheckpointWriteReport {
        let content = self.to_json();
        CheckpointWriteReport {
            artifact: self.clone(),
            path: path.to_string(),
            byte_count: content.len(),
            checksum: stable_checksum(&content),
            rehydration_status: "ready-from-receipts",
        }
    }
}

impl CheckpointWriteReport {
    pub fn artifact_json(&self) -> String {
        self.artifact.to_json()
    }

    pub fn to_json(&self) -> String {
        format!(
            concat!(
                "{{",
                "\"schema\":\"witness.checkpoint-write.v1\",",
                "\"path\":\"{}\",",
                "\"byte_count\":{},",
                "\"checksum\":{},",
                "\"rehydration_status\":\"{}\",",
                "\"artifact\":{}",
                "}}"
            ),
            escape_json(&self.path),
            self.byte_count,
            self.checksum,
            self.rehydration_status,
            self.artifact_json()
        )
    }
}

impl SessionCorpusReviewReport {
    pub fn approved(&self) -> bool {
        !self.raw_ingestion_allowed && self.gate_count == self.approved_gate_count
    }

    pub fn to_json(&self) -> String {
        let gates = self
            .gates
            .iter()
            .map(SessionCorpusGate::to_json)
            .collect::<Vec<_>>()
            .join(",");
        format!(
            concat!(
                "{{",
                "\"schema\":\"witness.session-corpus-review.v1\",",
                "\"suite\":\"{}\",",
                "\"purpose\":\"{}\",",
                "\"raw_ingestion_allowed\":{},",
                "\"gate_count\":{},",
                "\"approved_gate_count\":{},",
                "\"approved\":{},",
                "\"gates\":[{}]",
                "}}"
            ),
            escape_json(self.suite),
            escape_json(self.purpose),
            self.raw_ingestion_allowed,
            self.gate_count,
            self.approved_gate_count,
            self.approved(),
            gates
        )
    }
}

impl SessionCorpusGate {
    fn to_json(&self) -> String {
        format!(
            concat!(
                "{{",
                "\"id\":\"{}\",",
                "\"requirement\":\"{}\",",
                "\"status\":\"{}\",",
                "\"lattice_validation\":\"{}\"",
                "}}"
            ),
            escape_json(self.id),
            escape_json(self.requirement),
            escape_json(self.status),
            escape_json(self.lattice_validation)
        )
    }
}

impl MaximScenarioReport {
    pub fn to_json(&self) -> String {
        let scenarios = self
            .scenarios
            .iter()
            .map(MaximScenario::to_json)
            .collect::<Vec<_>>()
            .join(",");
        format!(
            concat!(
                "{{",
                "\"schema\":\"witness.maxim-scenarios.v1\",",
                "\"suite\":\"{}\",",
                "\"corpus\":\"{}\",",
                "\"custody\":\"{}\",",
                "\"scenario_count\":{},",
                "\"pointer_count\":{},",
                "\"receipt_count\":{},",
                "\"frontier_count\":{},",
                "\"scenarios\":[{}]",
                "}}"
            ),
            escape_json(self.suite),
            escape_json(self.corpus),
            escape_json(self.custody),
            self.scenario_count,
            self.pointer_count,
            self.receipt_count,
            self.frontier_count,
            scenarios
        )
    }
}

impl MaximScenario {
    fn to_json(&self) -> String {
        format!(
            concat!(
                "{{",
                "\"id\":\"{}\",",
                "\"user_goal\":\"{}\",",
                "\"source_pointer\":\"{}\",",
                "\"receipt\":\"{}\",",
                "\"lattice_validation\":\"{}\",",
                "\"result_shape\":\"{}\",",
                "\"saved_chunk\":\"{}\"",
                "}}"
            ),
            escape_json(self.id),
            escape_json(self.user_goal),
            escape_json(self.source_pointer),
            escape_json(self.receipt),
            escape_json(self.lattice_validation),
            escape_json(self.result_shape),
            escape_json(self.saved_chunk)
        )
    }
}

impl MaximConversationReport {
    pub fn to_json(&self) -> String {
        let conversations = self
            .conversations
            .iter()
            .map(MaximConversation::to_json)
            .collect::<Vec<_>>()
            .join(",");
        format!(
            concat!(
                "{{",
                "\"schema\":\"witness.maxim-conversations.v1\",",
                "\"suite\":\"{}\",",
                "\"corpus\":\"{}\",",
                "\"provider_mode\":\"{}\",",
                "\"custody\":\"{}\",",
                "\"semantic_owner\":\"{}\",",
                "\"conversation_count\":{},",
                "\"turn_count\":{},",
                "\"pointer_count\":{},",
                "\"receipt_count\":{},",
                "\"visual_update_count\":{},",
                "\"passed_gate_count\":{},",
                "\"conversations\":[{}]",
                "}}"
            ),
            escape_json(self.suite),
            escape_json(self.corpus),
            escape_json(self.provider_mode),
            escape_json(self.custody),
            escape_json(self.semantic_owner),
            self.conversation_count,
            self.turn_count,
            self.pointer_count,
            self.receipt_count,
            self.visual_update_count,
            self.passed_gate_count,
            conversations
        )
    }
}

impl MaximConversation {
    fn to_json(&self) -> String {
        let turns = self
            .turns
            .iter()
            .map(MaximConversationTurn::to_json)
            .collect::<Vec<_>>()
            .join(",");
        format!(
            concat!(
                "{{",
                "\"id\":\"{}\",",
                "\"title\":\"{}\",",
                "\"user_goal\":\"{}\",",
                "\"source_pointer\":\"{}\",",
                "\"receipt\":\"{}\",",
                "\"expected_shape\":\"{}\",",
                "\"evaluation_gate\":\"{}\",",
                "\"turns\":[{}]",
                "}}"
            ),
            escape_json(self.id),
            escape_json(self.title),
            escape_json(self.user_goal),
            escape_json(self.source_pointer),
            escape_json(self.receipt),
            escape_json(self.expected_shape),
            escape_json(self.evaluation_gate),
            turns
        )
    }
}

impl MaximConversationTurn {
    fn to_json(&self) -> String {
        format!(
            concat!(
                "{{",
                "\"id\":\"{}\",",
                "\"speaker\":\"{}\",",
                "\"summary\":\"{}\",",
                "\"harness_event\":\"{}\",",
                "\"visual_update\":\"{}\",",
                "\"lattice_anchor\":\"{}\"",
                "}}"
            ),
            escape_json(self.id),
            escape_json(self.speaker),
            escape_json(self.summary),
            escape_json(self.harness_event),
            escape_json(self.visual_update),
            escape_json(self.lattice_anchor)
        )
    }
}

impl MaximConversationEvaluationReport {
    pub fn to_json(&self) -> String {
        let results = self
            .results
            .iter()
            .map(MaximConversationEvaluation::to_json)
            .collect::<Vec<_>>()
            .join(",");
        format!(
            concat!(
                "{{",
                "\"schema\":\"witness.maxim-conversation-eval.v1\",",
                "\"suite\":\"{}\",",
                "\"corpus\":\"{}\",",
                "\"evaluation_target\":\"{}\",",
                "\"semantic_owner\":\"{}\",",
                "\"scenario_count\":{},",
                "\"passed_count\":{},",
                "\"failed_count\":{},",
                "\"gate_count\":{},",
                "\"results\":[{}]",
                "}}"
            ),
            escape_json(self.suite),
            escape_json(self.corpus),
            escape_json(self.evaluation_target),
            escape_json(self.semantic_owner),
            self.scenario_count,
            self.passed_count,
            self.failed_count,
            self.gate_count,
            results
        )
    }
}

impl MaximConversationEvaluation {
    fn to_json(&self) -> String {
        format!(
            concat!(
                "{{",
                "\"id\":\"{}\",",
                "\"title\":\"{}\",",
                "\"pointer_gate\":\"{}\",",
                "\"receipt_gate\":\"{}\",",
                "\"visual_gate\":\"{}\",",
                "\"frontier_gate\":\"{}\",",
                "\"shape_gate\":\"{}\",",
                "\"result\":\"{}\"",
                "}}"
            ),
            escape_json(self.id),
            escape_json(self.title),
            escape_json(self.pointer_gate),
            escape_json(self.receipt_gate),
            escape_json(self.visual_gate),
            escape_json(self.frontier_gate),
            escape_json(self.shape_gate),
            escape_json(self.result)
        )
    }
}

impl MaximConversationReplayReport {
    pub fn to_json(&self) -> String {
        let frames = self
            .frames
            .iter()
            .map(MaximConversationReplayFrame::to_json)
            .collect::<Vec<_>>()
            .join(",");
        format!(
            concat!(
                "{{",
                "\"schema\":\"witness.maxim-conversation-replay.v1\",",
                "\"suite\":\"{}\",",
                "\"transcript\":\"{}\",",
                "\"corpus\":\"{}\",",
                "\"semantic_owner\":\"{}\",",
                "\"frame_count\":{},",
                "\"visual_frame_count\":{},",
                "\"source_anchor_count\":{},",
                "\"frontier_frame_count\":{},",
                "\"frames\":[{}]",
                "}}"
            ),
            escape_json(self.suite),
            escape_json(self.transcript),
            escape_json(self.corpus),
            escape_json(self.semantic_owner),
            self.frame_count,
            self.visual_frame_count,
            self.source_anchor_count,
            self.frontier_frame_count,
            frames
        )
    }
}

impl MaximConversationReplayFrame {
    fn to_json(&self) -> String {
        format!(
            concat!(
                "{{",
                "\"turn_id\":\"{}\",",
                "\"speaker\":\"{}\",",
                "\"message\":\"{}\",",
                "\"expected_delta\":\"{}\",",
                "\"visual_frame\":\"{}\",",
                "\"lattice_anchor\":\"{}\",",
                "\"assertion\":\"{}\"",
                "}}"
            ),
            escape_json(self.turn_id),
            escape_json(self.speaker),
            escape_json(self.message),
            escape_json(self.expected_delta),
            escape_json(self.visual_frame),
            escape_json(self.lattice_anchor),
            escape_json(self.assertion)
        )
    }
}

impl MaximConversationSyncReport {
    pub fn to_json(&self) -> String {
        let checks = self
            .checks
            .iter()
            .map(MaximConversationSyncCheck::to_json)
            .collect::<Vec<_>>()
            .join(",");
        format!(
            concat!(
                "{{",
                "\"schema\":\"witness.maxim-conversation-sync.v1\",",
                "\"suite\":\"{}\",",
                "\"transcript\":\"{}\",",
                "\"sync_target\":\"{}\",",
                "\"semantic_owner\":\"{}\",",
                "\"check_count\":{},",
                "\"passed_count\":{},",
                "\"failed_count\":{},",
                "\"checks\":[{}]",
                "}}"
            ),
            escape_json(self.suite),
            escape_json(self.transcript),
            escape_json(self.sync_target),
            escape_json(self.semantic_owner),
            self.check_count,
            self.passed_count,
            self.failed_count,
            checks
        )
    }
}

impl MaximConversationSyncCheck {
    fn to_json(&self) -> String {
        format!(
            concat!(
                "{{",
                "\"id\":\"{}\",",
                "\"replay_frame\":\"{}\",",
                "\"chat_surface_state\":\"{}\",",
                "\"visual_contract\":\"{}\",",
                "\"expected_artifact\":\"{}\",",
                "\"sync_gate\":\"{}\",",
                "\"result\":\"{}\"",
                "}}"
            ),
            escape_json(self.id),
            escape_json(self.replay_frame),
            escape_json(self.chat_surface_state),
            escape_json(self.visual_contract),
            escape_json(self.expected_artifact),
            escape_json(self.sync_gate),
            escape_json(self.result)
        )
    }
}

impl ChatWebReadinessReport {
    pub fn to_json(&self) -> String {
        let routes = self
            .routes
            .iter()
            .map(ChatWebRoute::to_json)
            .collect::<Vec<_>>()
            .join(",");
        format!(
            concat!(
                "{{",
                "\"schema\":\"witness.chat-web-readiness.v1\",",
                "\"suite\":\"{}\",",
                "\"intended_command\":\"{}\",",
                "\"host_policy\":\"{}\",",
                "\"semantic_owner\":\"{}\",",
                "\"route_count\":{},",
                "\"data_feed_count\":{},",
                "\"privacy_gate_count\":{},",
                "\"ready_gate_count\":{},",
                "\"routes\":[{}]",
                "}}"
            ),
            escape_json(self.suite),
            escape_json(self.intended_command),
            escape_json(self.host_policy),
            escape_json(self.semantic_owner),
            self.route_count,
            self.data_feed_count,
            self.privacy_gate_count,
            self.ready_gate_count,
            routes
        )
    }
}

impl ChatWebRoute {
    fn to_json(&self) -> String {
        format!(
            concat!(
                "{{",
                "\"path\":\"{}\",",
                "\"surface\":\"{}\",",
                "\"data_feed\":\"{}\",",
                "\"render_state\":\"{}\",",
                "\"privacy_gate\":\"{}\",",
                "\"ready_gate\":\"{}\"",
                "}}"
            ),
            escape_json(self.path),
            escape_json(self.surface),
            escape_json(self.data_feed),
            escape_json(self.render_state),
            escape_json(self.privacy_gate),
            escape_json(self.ready_gate)
        )
    }
}

impl ChatWebFixtureBundleReport {
    pub fn to_json(&self) -> String {
        format!(
            concat!(
                "{{",
                "\"schema\":\"witness.chat-web-fixture.v1\",",
                "\"suite\":\"{}\",",
                "\"bootstrap_route\":\"{}\",",
                "\"host_policy\":\"{}\",",
                "\"semantic_owner\":\"{}\",",
                "\"feed_count\":{},",
                "\"privacy_gate\":\"{}\",",
                "\"feeds\":{{",
                "\"web_ready\":{},",
                "\"chat_surface\":{},",
                "\"maxim_replay\":{},",
                "\"maxim_sync\":{},",
                "\"build_meter\":{},",
                "\"artifact_assembly\":{},",
                "\"narrowing_trace\":{}",
                "}}",
                "}}"
            ),
            escape_json(self.suite),
            escape_json(self.bootstrap_route),
            escape_json(self.host_policy),
            escape_json(self.semantic_owner),
            self.feed_count,
            escape_json(self.privacy_gate),
            chat_web_readiness_fixture().to_json(),
            conversation_surface_fixture().to_json(),
            maxim_conversation_replay_fixture().to_json(),
            maxim_conversation_sync_fixture().to_json(),
            build_meter_fixture().to_json(),
            artifact_assembly_fixture().to_json(),
            narrowing_trace_fixture().to_json()
        )
    }
}

impl ChatWebStoryboardReport {
    pub fn to_json(&self) -> String {
        let panels = self
            .panels
            .iter()
            .map(ChatWebPanel::to_json)
            .collect::<Vec<_>>()
            .join(",");
        let frames = self
            .frames
            .iter()
            .map(ChatWebStoryboardFrame::to_json)
            .collect::<Vec<_>>()
            .join(",");
        format!(
            concat!(
                "{{",
                "\"schema\":\"witness.chat-web-storyboard.v1\",",
                "\"suite\":\"{}\",",
                "\"target_surface\":\"{}\",",
                "\"semantic_owner\":\"{}\",",
                "\"panel_count\":{},",
                "\"frame_count\":{},",
                "\"panels\":[{}],",
                "\"frames\":[{}]",
                "}}"
            ),
            escape_json(self.suite),
            escape_json(self.target_surface),
            escape_json(self.semantic_owner),
            self.panel_count,
            self.frame_count,
            panels,
            frames
        )
    }
}

impl ChatWebPanel {
    fn to_json(&self) -> String {
        format!(
            concat!(
                "{{",
                "\"id\":\"{}\",",
                "\"title\":\"{}\",",
                "\"role\":\"{}\",",
                "\"data_feed\":\"{}\",",
                "\"render_contract\":\"{}\"",
                "}}"
            ),
            escape_json(self.id),
            escape_json(self.title),
            escape_json(self.role),
            escape_json(self.data_feed),
            escape_json(self.render_contract)
        )
    }
}

impl ChatWebStoryboardFrame {
    fn to_json(&self) -> String {
        format!(
            concat!(
                "{{",
                "\"id\":\"{}\",",
                "\"replay_frame\":\"{}\",",
                "\"active_panel\":\"{}\",",
                "\"animation_cue\":\"{}\",",
                "\"user_feedback\":\"{}\",",
                "\"frontier_state\":\"{}\"",
                "}}"
            ),
            escape_json(self.id),
            escape_json(self.replay_frame),
            escape_json(self.active_panel),
            escape_json(self.animation_cue),
            escape_json(self.user_feedback),
            escape_json(self.frontier_state)
        )
    }
}

impl ChatWebAcceptanceReport {
    pub fn to_json(&self) -> String {
        let criteria = self
            .criteria
            .iter()
            .map(ChatWebAcceptanceCriterion::to_json)
            .collect::<Vec<_>>()
            .join(",");
        format!(
            concat!(
                "{{",
                "\"schema\":\"witness.chat-web-acceptance.v1\",",
                "\"suite\":\"{}\",",
                "\"target_command\":\"{}\",",
                "\"semantic_owner\":\"{}\",",
                "\"criterion_count\":{},",
                "\"required_count\":{},",
                "\"criteria\":[{}]",
                "}}"
            ),
            escape_json(self.suite),
            escape_json(self.target_command),
            escape_json(self.semantic_owner),
            self.criterion_count,
            self.required_count,
            criteria
        )
    }
}

impl ChatWebAcceptanceCriterion {
    fn to_json(&self) -> String {
        format!(
            concat!(
                "{{",
                "\"id\":\"{}\",",
                "\"requirement\":\"{}\",",
                "\"evidence_feed\":\"{}\",",
                "\"expected_check\":\"{}\",",
                "\"gate\":\"{}\"",
                "}}"
            ),
            escape_json(self.id),
            escape_json(self.requirement),
            escape_json(self.evidence_feed),
            escape_json(self.expected_check),
            escape_json(self.gate)
        )
    }
}

impl VisualContextReport {
    pub fn to_json(&self) -> String {
        let events = self
            .events
            .iter()
            .map(VisualContextEvent::to_json)
            .collect::<Vec<_>>()
            .join(",");
        format!(
            concat!(
                "{{",
                "\"schema\":\"witness.visual-context.v1\",",
                "\"suite\":\"{}\",",
                "\"scenario\":\"{}\",",
                "\"goal_artifact\":\"{}\",",
                "\"semantic_owner\":\"{}\",",
                "\"event_count\":{},",
                "\"saved_chunk_count\":{},",
                "\"frontier_count\":{},",
                "\"events\":[{}]",
                "}}"
            ),
            escape_json(self.suite),
            escape_json(self.scenario),
            escape_json(self.goal_artifact),
            escape_json(self.semantic_owner),
            self.event_count,
            self.saved_chunk_count,
            self.frontier_count,
            events
        )
    }
}

impl VisualContextEvent {
    fn to_json(&self) -> String {
        format!(
            concat!(
                "{{",
                "\"id\":\"{}\",",
                "\"motion\":\"{}\",",
                "\"lattice_contract\":\"{}\",",
                "\"user_feedback\":\"{}\",",
                "\"shape\":\"{}\"",
                "}}"
            ),
            escape_json(self.id),
            escape_json(self.motion),
            escape_json(self.lattice_contract),
            escape_json(self.user_feedback),
            escape_json(self.shape)
        )
    }
}

impl VisualShapeCatalog {
    pub fn to_json(&self) -> String {
        let shapes = self
            .shapes
            .iter()
            .map(VisualShape::to_json)
            .collect::<Vec<_>>()
            .join(",");
        format!(
            concat!(
                "{{",
                "\"schema\":\"witness.visual-shapes.v1\",",
                "\"suite\":\"{}\",",
                "\"semantic_owner\":\"{}\",",
                "\"shape_count\":{},",
                "\"shapes\":[{}]",
                "}}"
            ),
            escape_json(self.suite),
            escape_json(self.semantic_owner),
            self.shape_count,
            shapes
        )
    }
}

impl VisualShape {
    fn to_json(&self) -> String {
        format!(
            concat!(
                "{{",
                "\"id\":\"{}\",",
                "\"scenario\":\"{}\",",
                "\"artifact_shape\":\"{}\",",
                "\"user_satisfaction\":\"{}\",",
                "\"required_lattice_evidence\":\"{}\",",
                "\"saveable_chunk\":\"{}\"",
                "}}"
            ),
            escape_json(self.id),
            escape_json(self.scenario),
            escape_json(self.artifact_shape),
            escape_json(self.user_satisfaction),
            escape_json(self.required_lattice_evidence),
            escape_json(self.saveable_chunk)
        )
    }
}

impl ChunkInventoryReport {
    pub fn to_json(&self) -> String {
        let chunks = self
            .chunks
            .iter()
            .map(ContextChunk::to_json)
            .collect::<Vec<_>>()
            .join(",");
        format!(
            concat!(
                "{{",
                "\"schema\":\"witness.chunk-inventory.v1\",",
                "\"suite\":\"{}\",",
                "\"semantic_owner\":\"{}\",",
                "\"inventory_status\":\"{}\",",
                "\"chunk_count\":{},",
                "\"reusable_count\":{},",
                "\"frontier_count\":{},",
                "\"chunks\":[{}]",
                "}}"
            ),
            escape_json(self.suite),
            escape_json(self.semantic_owner),
            escape_json(self.inventory_status),
            self.chunk_count,
            self.reusable_count,
            self.frontier_count,
            chunks
        )
    }
}

impl ContextChunk {
    fn to_json(&self) -> String {
        format!(
            concat!(
                "{{",
                "\"id\":\"{}\",",
                "\"title\":\"{}\",",
                "\"source\":\"{}\",",
                "\"artifact_shape\":\"{}\",",
                "\"lattice_anchor\":\"{}\",",
                "\"custody\":\"{}\",",
                "\"reuse_status\":\"{}\"",
                "}}"
            ),
            escape_json(self.id),
            escape_json(self.title),
            escape_json(self.source),
            escape_json(self.artifact_shape),
            escape_json(self.lattice_anchor),
            escape_json(self.custody),
            escape_json(self.reuse_status)
        )
    }
}

impl BuildMeterReport {
    pub fn to_json(&self) -> String {
        let stages = self
            .stages
            .iter()
            .map(BuildMeterStage::to_json)
            .collect::<Vec<_>>()
            .join(",");
        format!(
            concat!(
                "{{",
                "\"schema\":\"witness.build-meter.v1\",",
                "\"suite\":\"{}\",",
                "\"scenario\":\"{}\",",
                "\"semantic_owner\":\"{}\",",
                "\"overall_progress\":{},",
                "\"stage_count\":{},",
                "\"stages\":[{}]",
                "}}"
            ),
            escape_json(self.suite),
            escape_json(self.scenario),
            escape_json(self.semantic_owner),
            self.overall_progress,
            self.stage_count,
            stages
        )
    }
}

impl BuildMeterStage {
    fn to_json(&self) -> String {
        format!(
            concat!(
                "{{",
                "\"id\":\"{}\",",
                "\"label\":\"{}\",",
                "\"progress\":{},",
                "\"signal\":\"{}\",",
                "\"lattice_anchor\":\"{}\",",
                "\"visual_feedback\":\"{}\"",
                "}}"
            ),
            escape_json(self.id),
            escape_json(self.label),
            self.progress,
            escape_json(self.signal),
            escape_json(self.lattice_anchor),
            escape_json(self.visual_feedback)
        )
    }
}

impl ArtifactAssemblyReport {
    pub fn to_json(&self) -> String {
        let pieces = self
            .pieces
            .iter()
            .map(ArtifactPiece::to_json)
            .collect::<Vec<_>>()
            .join(",");
        format!(
            concat!(
                "{{",
                "\"schema\":\"witness.artifact-assembly.v1\",",
                "\"suite\":\"{}\",",
                "\"scenario\":\"{}\",",
                "\"artifact_shape\":\"{}\",",
                "\"semantic_owner\":\"{}\",",
                "\"piece_count\":{},",
                "\"included_count\":{},",
                "\"frontier_count\":{},",
                "\"pieces\":[{}]",
                "}}"
            ),
            escape_json(self.suite),
            escape_json(self.scenario),
            escape_json(self.artifact_shape),
            escape_json(self.semantic_owner),
            self.piece_count,
            self.included_count,
            self.frontier_count,
            pieces
        )
    }
}

impl ArtifactPiece {
    fn to_json(&self) -> String {
        format!(
            concat!(
                "{{",
                "\"id\":\"{}\",",
                "\"label\":\"{}\",",
                "\"role\":\"{}\",",
                "\"lattice_anchor\":\"{}\",",
                "\"fit_status\":\"{}\",",
                "\"visual_feedback\":\"{}\"",
                "}}"
            ),
            escape_json(self.id),
            escape_json(self.label),
            escape_json(self.role),
            escape_json(self.lattice_anchor),
            escape_json(self.fit_status),
            escape_json(self.visual_feedback)
        )
    }
}

impl NarrowingTraceReport {
    pub fn to_json(&self) -> String {
        let steps = self
            .steps
            .iter()
            .map(NarrowingStep::to_json)
            .collect::<Vec<_>>()
            .join(",");
        format!(
            concat!(
                "{{",
                "\"schema\":\"witness.narrowing-trace.v1\",",
                "\"suite\":\"{}\",",
                "\"scenario\":\"{}\",",
                "\"semantic_owner\":\"{}\",",
                "\"initial_candidate_count\":{},",
                "\"final_candidate_count\":{},",
                "\"saved_chunk_count\":{},",
                "\"frontier_count\":{},",
                "\"steps\":[{}]",
                "}}"
            ),
            escape_json(self.suite),
            escape_json(self.scenario),
            escape_json(self.semantic_owner),
            self.initial_candidate_count,
            self.final_candidate_count,
            self.saved_chunk_count,
            self.frontier_count,
            steps
        )
    }
}

impl NarrowingStep {
    fn to_json(&self) -> String {
        format!(
            concat!(
                "{{",
                "\"id\":\"{}\",",
                "\"query\":\"{}\",",
                "\"lattice_operation\":\"{}\",",
                "\"before_count\":{},",
                "\"after_count\":{},",
                "\"saved_chunk\":\"{}\",",
                "\"frontier\":\"{}\",",
                "\"visual_feedback\":\"{}\"",
                "}}"
            ),
            escape_json(self.id),
            escape_json(self.query),
            escape_json(self.lattice_operation),
            self.before_count,
            self.after_count,
            escape_json(self.saved_chunk),
            escape_json(self.frontier),
            escape_json(self.visual_feedback)
        )
    }
}

impl ConversationSurfaceReport {
    pub fn to_json(&self) -> String {
        let turns = self
            .turns
            .iter()
            .map(ConversationTurn::to_json)
            .collect::<Vec<_>>()
            .join(",");
        format!(
            concat!(
                "{{",
                "\"schema\":\"witness.conversation-surface.v1\",",
                "\"suite\":\"{}\",",
                "\"scenario\":\"{}\",",
                "\"primary_surface\":\"{}\",",
                "\"companion_surface\":\"{}\",",
                "\"semantic_owner\":\"{}\",",
                "\"turn_count\":{},",
                "\"visual_update_count\":{},",
                "\"frontier_count\":{},",
                "\"turns\":[{}]",
                "}}"
            ),
            escape_json(self.suite),
            escape_json(self.scenario),
            escape_json(self.primary_surface),
            escape_json(self.companion_surface),
            escape_json(self.semantic_owner),
            self.turn_count,
            self.visual_update_count,
            self.frontier_count,
            turns
        )
    }
}

impl ConversationTurn {
    fn to_json(&self) -> String {
        format!(
            concat!(
                "{{",
                "\"id\":\"{}\",",
                "\"speaker\":\"{}\",",
                "\"message_summary\":\"{}\",",
                "\"harness_event\":\"{}\",",
                "\"visual_state\":\"{}\",",
                "\"lattice_anchor\":\"{}\",",
                "\"frontier\":\"{}\"",
                "}}"
            ),
            escape_json(self.id),
            escape_json(self.speaker),
            escape_json(self.message_summary),
            escape_json(self.harness_event),
            escape_json(self.visual_state),
            escape_json(self.lattice_anchor),
            escape_json(self.frontier)
        )
    }
}

impl ResponseDeltaPlan {
    pub fn delta_count(&self) -> usize {
        self.deltas.len()
    }

    pub fn folded_event_count(&self) -> usize {
        self.deltas.len()
    }

    pub fn to_json(&self) -> String {
        self.to_json_with_schema("witness.response-deltas.v1")
    }

    pub fn to_lattice_delta_projection_json(&self) -> String {
        self.to_json_with_schema("witness.lattice-delta-projection.v1")
    }

    fn to_json_with_schema(&self, schema: &str) -> String {
        let deltas = self
            .deltas
            .iter()
            .map(ResponseDelta::to_json)
            .collect::<Vec<_>>()
            .join(",");
        let folded_events = self
            .deltas
            .iter()
            .map(ResponseDelta::folded_event_json)
            .collect::<Vec<_>>()
            .join(",");
        format!(
            concat!(
                "{{",
                "\"schema\":\"{}\",",
                "\"language\":\"LATTICE\",",
                "\"language_owner\":\"LATTICE\",",
                "\"adapter\":\"WITNESS\",",
                "\"projection\":\"harness-intent-deltas\",",
                "\"fixture\":\"{}\",",
                "\"source_response\":\"{}\",",
                "\"active_cut\":\"{}\",",
                "\"folded_cut\":\"{}\",",
                "\"delta_count\":{},",
                "\"folded_event_count\":{},",
                "\"deltas\":[{}],",
                "\"folded_events\":[{}]",
                "}}"
            ),
            schema,
            escape_json(self.fixture),
            escape_json(self.source_response),
            escape_json(self.active_cut),
            escape_json(self.folded_cut),
            self.delta_count(),
            self.folded_event_count(),
            deltas,
            folded_events
        )
    }
}

impl ResponseDelta {
    fn to_json(&self) -> String {
        format!(
            concat!(
                "{{",
                "\"id\":\"{}\",",
                "\"kind\":\"{}\",",
                "\"expression\":\"{}\",",
                "\"folded_operation\":\"{}\",",
                "\"summary\":\"{}\"",
                "}}"
            ),
            escape_json(self.id),
            self.kind.as_str(),
            escape_json(self.expression),
            self.kind.folded_operation(),
            escape_json(self.summary)
        )
    }

    fn folded_event_json(&self) -> String {
        let event_kind = match self.kind {
            ResponseDeltaKind::NarrowContext | ResponseDeltaKind::JoinContext => {
                HarnessEventKind::LatticeOperation
            }
            _ => HarnessEventKind::ContextDelta,
        };
        format!(
            concat!(
                "{{",
                "\"id\":\"folded-{}\",",
                "\"kind\":\"{}\",",
                "\"operation\":\"{}\",",
                "\"receipt\":\"receipt:{}\"",
                "}}"
            ),
            escape_json(self.id),
            event_kind.as_str(),
            self.kind.folded_operation(),
            escape_json(self.id)
        )
    }
}

impl HarnessReplay {
    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    pub fn source_pointer_count(&self) -> usize {
        self.events
            .iter()
            .filter(|event| event.source_pointer.is_some())
            .count()
    }

    pub fn receipt_count(&self) -> usize {
        self.events.len()
    }

    pub fn to_json(&self) -> String {
        let events = self
            .events
            .iter()
            .map(HarnessEvent::to_json)
            .collect::<Vec<_>>()
            .join(",");
        format!(
            concat!(
                "{{",
                "\"schema\":\"witness.harness.v1\",",
                "\"fixture\":\"{}\",",
                "\"substrate\":\"{}\",",
                "\"active_cut\":\"{}\",",
                "\"checkpoint\":\"{}\",",
                "\"rehydrated_cut\":\"{}\",",
                "\"event_count\":{},",
                "\"source_pointer_count\":{},",
                "\"receipt_count\":{},",
                "\"frontier_count\":{},",
                "\"events\":[{}]",
                "}}"
            ),
            escape_json(self.fixture),
            escape_json(self.substrate),
            escape_json(self.active_cut),
            escape_json(self.checkpoint),
            escape_json(self.rehydrated_cut),
            self.event_count(),
            self.source_pointer_count(),
            self.receipt_count(),
            self.frontier_count,
            events
        )
    }
}

impl HarnessEvent {
    fn to_json(&self) -> String {
        format!(
            concat!(
                "{{",
                "\"id\":\"{}\",",
                "\"kind\":\"{}\",",
                "\"summary\":\"{}\",",
                "\"source_pointer\":{},",
                "\"receipt\":\"{}\"",
                "}}"
            ),
            escape_json(self.id),
            self.kind.as_str(),
            escape_json(self.summary),
            optional_json_string(self.source_pointer),
            escape_json(self.receipt)
        )
    }
}

pub fn claude_session_fixture() -> HarnessReplay {
    HarnessReplay {
        fixture: "claude-session",
        substrate: "LATTICE closed-cut contract",
        active_cut: "cut:WITNESS:claude-session:active",
        checkpoint: "checkpoint:WITNESS:claude-session:001",
        rehydrated_cut: "cut:WITNESS:claude-session:rehydrated",
        frontier_count: 2,
        events: vec![
            HarnessEvent {
                id: "bootstrap-context",
                kind: HarnessEventKind::BootstrapContext,
                summary: "Load repo instructions, product plan, wave docs, and dependency boundary as the initial policy cut.",
                source_pointer: Some("repo:CLAUDE.md+CONTEXT.md+context/waves/PHASES.md"),
                receipt: "receipt:bootstrap-context",
            },
            HarnessEvent {
                id: "user-turn",
                kind: HarnessEventKind::UserTurn,
                summary: "Capture the user task as an intent grain linked to the active policy cut.",
                source_pointer: None,
                receipt: "receipt:user-turn",
            },
            HarnessEvent {
                id: "file-read",
                kind: HarnessEventKind::FileRead,
                summary: "Record every file read as source-pointer evidence with path, reason, and version.",
                source_pointer: Some("repo:crates/witness-core/src/lib.rs"),
                receipt: "receipt:file-read",
            },
            HarnessEvent {
                id: "file-edit",
                kind: HarnessEventKind::FileEdit,
                summary: "Record the edit as a transition grain with prior state, target invariant, and validation owed.",
                source_pointer: Some("repo:crates/witness-cli/src/main.rs"),
                receipt: "receipt:file-edit",
            },
            HarnessEvent {
                id: "validation",
                kind: HarnessEventKind::Validation,
                summary: "Attach validation output to the active cut before declaring the harness step complete.",
                source_pointer: None,
                receipt: "receipt:validation",
            },
            HarnessEvent {
                id: "checkpoint",
                kind: HarnessEventKind::Checkpoint,
                summary: "Persist active cut hash, receipts, frontier, and validation status for later resume.",
                source_pointer: None,
                receipt: "receipt:checkpoint",
            },
            HarnessEvent {
                id: "rehydrate",
                kind: HarnessEventKind::Rehydrate,
                summary: "Rebuild the active context from receipts instead of replaying raw chat history.",
                source_pointer: None,
                receipt: "receipt:rehydrate",
            },
        ],
    }
}

pub fn response_delta_fixture() -> ResponseDeltaPlan {
    ResponseDeltaPlan {
        fixture: "claude-session-response",
        source_response:
            "AI proposes next harness work without writing low-level lattice expressions.",
        active_cut: "cut:WITNESS:claude-session:active",
        folded_cut: "cut:WITNESS:claude-session:after-response-deltas",
        deltas: vec![
            ResponseDelta {
                id: "keep-task-intent",
                kind: ResponseDeltaKind::KeepIntent,
                expression: "keep current task intent",
                summary: "Preserve the user's goal as the active intent cut.",
            },
            ResponseDelta {
                id: "add-maxim-source",
                kind: ResponseDeltaKind::AddSource,
                expression: "add MAXIM source-corpus pointers",
                summary: "Select MAXIM as candidate context without copying source content.",
            },
            ResponseDelta {
                id: "narrow-to-WITNESS-roles",
                kind: ResponseDeltaKind::NarrowContext,
                expression: "focus on WITNESS role guidance",
                summary: "Fold to a meet with the WITNESS role constraint.",
            },
            ResponseDelta {
                id: "join-lattice-principle",
                kind: ResponseDeltaKind::JoinContext,
                expression: "include LATTICE context principle",
                summary: "Fold to a join with the LATTICE organizing-context cut.",
            },
            ResponseDelta {
                id: "frontier-provider-adapters",
                kind: ResponseDeltaKind::FrontierGap,
                expression: "defer provider adapters",
                summary: "Keep provider execution on frontier until the canonical loop is stable.",
            },
            ResponseDelta {
                id: "validation-owed",
                kind: ResponseDeltaKind::ValidationOwed,
                expression: "validate response-delta folding",
                summary: "Require tests and CLI JSON before checkpointing.",
            },
            ResponseDelta {
                id: "checkpoint-after-fold",
                kind: ResponseDeltaKind::Checkpoint,
                expression: "checkpoint folded context",
                summary: "Persist the folded cut and receipts for rehydration.",
            },
        ],
    }
}

pub fn lattice_handoff_fixture() -> LatticeHandoffReport {
    LatticeHandoffReport {
        fixture: "claude-session",
        language_owner: "LATTICE",
        boundary: "WITNESS displays handoff requests and LATTICE replay responses; LATTICE owns closure, budgets, meet/join execution, and receipt evidence.",
        lattice_replay_command: "lattice-cli handoff replay tiny --json",
        lattice_replay_schema: "lattice.handoff-replay.v1",
        lattice_delta_schema: "lattice.delta.v1",
        lattice_pack_id: "pack:witness-handoff-pack:witness-handoff-replay",
        lattice_prompt_frame_version: "lattice.prompt-frame.v1",
        lattice_frontier_status: "frontier_deferred",
        lattice_frontier_count: 15,
        lattice_receipt_citation:
            "LATTICE receipt lattice-stable-v1:e44e5cc6a36f0cf3",
        source_pointers: vec![
            LatticeSourcePointer {
                id: "source:bootstrap-policy",
                target: "repo:CLAUDE.md+CONTEXT.md+context/waves/PHASES.md",
                evidence_role: "policy-cut-input",
                custody: "pointer-only",
            },
            LatticeSourcePointer {
                id: "source:file-read",
                target: "repo:crates/witness-core/src/lib.rs",
                evidence_role: "implementation-evidence",
                custody: "pointer-only",
            },
            LatticeSourcePointer {
                id: "source:file-edit",
                target: "repo:crates/witness-cli/src/main.rs",
                evidence_role: "transition-evidence",
                custody: "pointer-only",
            },
        ],
        cuts: vec![
            LatticeCut {
                id: "cut:WITNESS:claude-session:active",
                kind: "active-context-request",
                source: "bootstrap-context",
                receipt: "receipt:bootstrap-context",
            },
            LatticeCut {
                id: "cut:WITNESS:claude-session:after-response-deltas",
                kind: "folded-context-request",
                source: "response-delta-plan",
                receipt: "receipt:checkpoint-after-fold",
            },
            LatticeCut {
                id: "cut:WITNESS:claude-session:rehydrated",
                kind: "rehydration-request",
                source: "checkpoint:WITNESS:claude-session:001",
                receipt: "receipt:rehydrate",
            },
        ],
        receipts: vec![
            LatticeReceipt {
                id: "receipt:bootstrap-context",
                event: "bootstrap-context",
                owed: "policy-source-pointer-recorded",
            },
            LatticeReceipt {
                id: "receipt:user-turn",
                event: "user-turn",
                owed: "intent-grain-recorded",
            },
            LatticeReceipt {
                id: "receipt:file-read",
                event: "file-read",
                owed: "source-pointer-recorded",
            },
            LatticeReceipt {
                id: "receipt:file-edit",
                event: "file-edit",
                owed: "transition-recorded",
            },
            LatticeReceipt {
                id: "receipt:validation",
                event: "validation",
                owed: "validation-output-attached",
            },
            LatticeReceipt {
                id: "receipt:checkpoint",
                event: "checkpoint",
                owed: "checkpoint-state-persisted",
            },
            LatticeReceipt {
                id: "receipt:rehydrate",
                event: "rehydrate",
                owed: "rehydration-from-receipts",
            },
        ],
        frontiers: vec![
            LatticeFrontier {
                id: "frontier:provider-adapters",
                reason: "Provider execution remains deferred in the foundation wave.",
                next_gate: "provider-adapter-review",
            },
            LatticeFrontier {
                id: "frontier:lattice-execution",
                reason: "LATTICE replay returned closed-cut and budget evidence; broader fixtures remain deferred.",
                next_gate: "broader-lattice-handoff-fixtures",
            },
        ],
        operation_intents: vec![
            LatticeOperationIntent {
                id: "intent:meet-with-constraint",
                operation: "meet",
                source_delta: "narrow-to-WITNESS-roles",
                status: "handoff-request",
            },
            LatticeOperationIntent {
                id: "intent:join-with-source-cut",
                operation: "join",
                source_delta: "join-lattice-principle",
                status: "handoff-request",
            },
        ],
    }
}

pub fn lattice_source_pilot_fixture() -> LatticeSourcePilotReport {
    LatticeSourcePilotReport {
        schema: "witness.lattice-source-pilot.v1",
        consumed_command: "lattice-cli registry pilot fontes --workspace <path> --json",
        semantic_owner: "LATTICE",
        viewer_role: "pointer-only evidence display",
        source_id: "fontes:apache-calcite:query-planning",
        owner_repo: "giodl73-repo/FONTES",
        work_id: "fontes:apache-calcite:query-planning",
        fletch_registry_path:
            ".fletch\\registries\\fontes-apache-calcite-query-planning-surfaces.json",
        fletch_id: "fontes.apache-calcite.lattice",
        proof_ledger_path: "sources\\tables\\proof-source-ledger.json",
        proof_record_path: ".proof\\sources\\fontes-course-source-ledger.source.md",
        rights_policy: "derived_text_allowed",
        custody_receipt: "registry-add-fontes-apache-calcite-query-planning-0000000002.json",
        metadata_only: true,
        source_content_copied: false,
        display_lens: "registered source metadata, custody receipt, and evidence pointer only",
    }
}

pub fn mechanical_sandbox_fixture() -> MechanicalSandboxReport {
    MechanicalSandboxReport {
        schema: "witness.mechanical-sandbox.v1",
        fixture: "literal-condition-preview",
        provider_execution: "none-fixture-backed",
        preset_dataset: "vacation-literal-candidates",
        user_condition: "prefer quiet parks and defer crowded campgrounds",
        consumed_lattice_schema: "lattice.delta.v1",
        lattice_delta_expression:
            "meet(active_literal_cut, condition:quiet-parks) + frontier(crowded-campgrounds)",
        folded_operation: "meet-with-constraint",
        reviewed_turn_boundary:
            "reviewed-turn-only: user condition is shown before any LATTICE handoff",
        receipt_citation:
            "WITNESS cites LATTICE handoff receipt after review; no provider call is made.",
        accepted_delta_count: 1,
        deferred_delta_count: 1,
        handoff_ready: true,
    }
}

pub fn reviewer_workflow_fixture() -> ReviewerWorkflowReport {
    ReviewerWorkflowReport {
        schema: "witness.reviewer-workflow.v1",
        default_route: "handoff-review",
        semantic_owner: "LATTICE",
        viewer_role: "review workflow defaults",
        default_graph_view: "evidence",
        default_review_path: vec![
            "boundary-view",
            "evidence-trail",
            "algebra-proof",
            "receipt-citation",
            "frontier-obligations",
        ],
        receipt_required: true,
        provider_execution: "none-review-only",
    }
}

pub fn press_preview_fixture() -> PressPreviewReport {
    PressPreviewReport {
        schema: "witness.press-preview.v1",
        consumed_command: "lattice-cli handoff replay tiny --json",
        consumed_contract_schema: "lattice.press-frame.v1",
        semantic_owner: "LATTICE",
        preview_owner: "WITNESS",
        renderer_owner: "PRESS",
        press_frame_id: "press-frame:witness-handoff-replay",
        preview_scope: "display LATTICE PRESS-frame metadata and downstream format intent only",
        preview_formats: vec!["md", "html", "json-report"],
        witness_renders_publication: false,
        lattice_renders_publication: false,
    }
}

pub fn lattice_shard_display_fixture() -> LatticeShardDisplayReport {
    LatticeShardDisplayReport {
        schema: "witness.lattice-shard-display.v1",
        consumed_command: "lattice-cli materialize shards demo --json",
        consumed_contract_schema: "lattice.shard-alignment-demo.v1",
        semantic_owner: "LATTICE",
        viewer_role: "display-only shard alignment review surface",
        boundary: "WITNESS displays LATTICE-owned shard manifests, boundary edges, alignment rows, and aggregate hash without recomputing closure or alignment.",
        aggregate_hash: "lattice-stable-v1:d2b94725746a2216",
        shard_count: 5,
        boundary_edge_count: 6,
        alignment_row_count: 5,
        frontier_count: 4,
        conflicting_boundary_count: 1,
        deterministic_replay: true,
        witness_recomputes_alignment: false,
        display_modes: vec![
            "shard-boxes",
            "boundary-edges",
            "alignment-rows",
            "frontier-conflicts",
            "receipt-citations",
        ],
        shards: vec![
            LatticeShardDisplayShard {
                shard_id: "search-docs-us-2026-q2",
                source_scope: "search/docs/us/2026-q2",
                rights_policy: "derived_text_allowed",
                status: "closed",
                grain_count: 24_000,
                bond_count: 120_000,
                receipt_count: 4,
                frontier_count: 0,
                manifest_hash: "lattice-stable-v1:6534d70777305b73",
            },
            LatticeShardDisplayShard {
                shard_id: "search-docs-eu-2026-q2",
                source_scope: "search/docs/eu/2026-q2",
                rights_policy: "derived_text_allowed",
                status: "closed",
                grain_count: 24_000,
                bond_count: 120_000,
                receipt_count: 4,
                frontier_count: 0,
                manifest_hash: "lattice-stable-v1:d5658552f6625155",
            },
            LatticeShardDisplayShard {
                shard_id: "support-kb-2026-q2",
                source_scope: "support/kb/2026-q2",
                rights_policy: "derived_text_allowed",
                status: "closed",
                grain_count: 12_000,
                bond_count: 64_000,
                receipt_count: 3,
                frontier_count: 0,
                manifest_hash: "lattice-stable-v1:05b0663b4f5f964a",
            },
            LatticeShardDisplayShard {
                shard_id: "engineering-notes-2026-q2",
                source_scope: "engineering/notes/2026-q2",
                rights_policy: "internal_reference_only",
                status: "frontier_deferred",
                grain_count: 18_000,
                bond_count: 90_000,
                receipt_count: 5,
                frontier_count: 2,
                manifest_hash: "lattice-stable-v1:59af038837acd227",
            },
            LatticeShardDisplayShard {
                shard_id: "release-artifacts-2026-q2",
                source_scope: "release/artifacts/2026-q2",
                rights_policy: "derived_text_allowed",
                status: "closed",
                grain_count: 9_000,
                bond_count: 45_000,
                receipt_count: 2,
                frontier_count: 0,
                manifest_hash: "lattice-stable-v1:8164ef7b483bcdcf",
            },
        ],
        boundary_edges: vec![
            LatticeShardDisplayEdge {
                edge_id: "edge-search-us-eu-same-entity",
                from_shard: "search-docs-us-2026-q2",
                to_shard: "search-docs-eu-2026-q2",
                kind: "same_entity",
                evidence_receipt: "receipt:boundary:search-us-eu",
                rights_compatible: true,
                alignment_status: "aligned",
            },
            LatticeShardDisplayEdge {
                edge_id: "edge-search-us-support-bridge",
                from_shard: "search-docs-us-2026-q2",
                to_shard: "support-kb-2026-q2",
                kind: "bridge",
                evidence_receipt: "receipt:boundary:search-us-support",
                rights_compatible: true,
                alignment_status: "aligned",
            },
            LatticeShardDisplayEdge {
                edge_id: "edge-search-eu-release-cites",
                from_shard: "search-docs-eu-2026-q2",
                to_shard: "release-artifacts-2026-q2",
                kind: "cites",
                evidence_receipt: "receipt:boundary:search-eu-release",
                rights_compatible: true,
                alignment_status: "aligned",
            },
            LatticeShardDisplayEdge {
                edge_id: "edge-release-search-us-bridge",
                from_shard: "release-artifacts-2026-q2",
                to_shard: "search-docs-us-2026-q2",
                kind: "bridge",
                evidence_receipt: "receipt:boundary:release-search-us",
                rights_compatible: true,
                alignment_status: "aligned",
            },
            LatticeShardDisplayEdge {
                edge_id: "edge-engineering-release-frontier",
                from_shard: "engineering-notes-2026-q2",
                to_shard: "release-artifacts-2026-q2",
                kind: "cites",
                evidence_receipt: "receipt:boundary:engineering-release",
                rights_compatible: false,
                alignment_status: "frontier",
            },
            LatticeShardDisplayEdge {
                edge_id: "edge-support-engineering-conflict",
                from_shard: "support-kb-2026-q2",
                to_shard: "engineering-notes-2026-q2",
                kind: "conflicts",
                evidence_receipt: "receipt:boundary:support-engineering",
                rights_compatible: false,
                alignment_status: "conflict",
            },
        ],
        alignment_rows: vec![
            LatticeShardDisplayRow {
                row_id: "all_shards_closed_or_frontiered",
                passed: true,
                evidence: "all demo shards carry receipts and no shard claims budget-failure success",
            },
            LatticeShardDisplayRow {
                row_id: "boundary_edges_typed",
                passed: true,
                evidence: "six fixed boundary edges use bridge/cites/same_entity/conflicts kinds",
            },
            LatticeShardDisplayRow {
                row_id: "rights_compatibility_preserved",
                passed: true,
                evidence: "internal engineering-note edges remain frontier/conflict instead of aligned",
            },
            LatticeShardDisplayRow {
                row_id: "aggregate_hash_replays",
                passed: true,
                evidence: "aggregate hash is cited from LATTICE and displayed without recomputation",
            },
            LatticeShardDisplayRow {
                row_id: "conflicts_not_flattened",
                passed: true,
                evidence: "support/engineering conflict is preserved as a boundary row",
            },
        ],
    }
}

pub fn lattice_shard_scenarios_fixture() -> LatticeShardScenarioDisplayReport {
    LatticeShardScenarioDisplayReport {
        schema: "witness.lattice-shard-scenarios.v1",
        consumed_command: "lattice-cli shards scenarios --json",
        consumed_contract_schema: "lattice.shard-scenario-set.v1",
        semantic_owner: "LATTICE",
        viewer_role: "display-only shard scenario review surface",
        boundary: "WITNESS displays LATTICE-owned shard scenario examples without recomputing routing, closure, or boundary semantics.",
        scenario_count: 5,
        scenario_hash: "lattice-stable-v1:ab04b3ad46ccdc5f",
        witness_recomputes_routing: false,
        examples: vec![
            LatticeShardScenarioDisplayExample {
                scenario_id: "release-readiness-route",
                title: "Release readiness route",
                selected_shards: vec!["release-artifacts-2026-q2", "search-docs-us-2026-q2"],
                frontier_shards: Vec::new(),
                boundary_edges: vec![
                    "edge-release-search-us-bridge",
                    "edge-search-eu-release-cites",
                ],
                expected_outcome: "route release and search summaries, then cite boundary receipts",
            },
            LatticeShardScenarioDisplayExample {
                scenario_id: "support-conflict-route",
                title: "Support conflict route",
                selected_shards: vec!["support-kb-2026-q2", "release-artifacts-2026-q2"],
                frontier_shards: vec!["engineering-notes-2026-q2"],
                boundary_edges: vec![
                    "edge-support-engineering-conflict",
                    "edge-engineering-release-frontier",
                ],
                expected_outcome:
                    "select support evidence and preserve engineering notes as rights frontier",
            },
            LatticeShardScenarioDisplayExample {
                scenario_id: "regional-search-bridge",
                title: "Regional search bridge",
                selected_shards: vec!["search-docs-us-2026-q2", "search-docs-eu-2026-q2"],
                frontier_shards: Vec::new(),
                boundary_edges: vec!["edge-search-us-eu-same-entity"],
                expected_outcome:
                    "bridge same-entity search shards without merging raw shard interiors",
            },
            LatticeShardScenarioDisplayExample {
                scenario_id: "rights-frontier-review",
                title: "Rights frontier review",
                selected_shards: vec!["release-artifacts-2026-q2"],
                frontier_shards: vec!["engineering-notes-2026-q2"],
                boundary_edges: vec!["edge-engineering-release-frontier"],
                expected_outcome: "keep internal engineering notes visible as a frontier obligation",
            },
            LatticeShardScenarioDisplayExample {
                scenario_id: "coverage-gap-sweep",
                title: "Coverage gap sweep",
                selected_shards: vec![
                    "release-artifacts-2026-q2",
                    "support-kb-2026-q2",
                    "search-docs-us-2026-q2",
                ],
                frontier_shards: vec!["engineering-notes-2026-q2"],
                boundary_edges: vec![
                    "edge-search-us-support-bridge",
                    "edge-release-search-us-bridge",
                    "edge-support-engineering-conflict",
                ],
                expected_outcome:
                    "select covered summaries and report engineering conflict as unresolved coverage",
            },
        ],
    }
}

pub fn lattice_shard_validation_fixture() -> LatticeShardValidationDisplayReport {
    LatticeShardValidationDisplayReport {
        schema: "witness.lattice-shard-validation.v1",
        consumed_command: "lattice-cli validate shards --workspace .lattice-shards --json",
        consumed_contract_schema: "lattice.shard-artifact-validation.v1",
        semantic_owner: "LATTICE",
        viewer_role: "display-only shard artifact validation review surface",
        boundary: "WITNESS displays LATTICE-owned shard artifact validation checks without reading manifests, replaying hashes, scanning raw grains, or claiming global closure.",
        aggregate_hash: "lattice-stable-v1:d2b94725746a2216",
        manifest_path: ".lattice-shards\\cache\\shards-demo-manifest.json",
        receipt_path: ".lattice-shards\\receipts\\shards-demo-0000000002.json",
        check_count: 6,
        validation_passed: true,
        witness_recomputes_validation: false,
        checks: vec![
            LatticeShardValidationDisplayCheck {
                name: "workspace-directories",
                passed: true,
                note: "workspace directories are present before artifact review",
            },
            LatticeShardValidationDisplayCheck {
                name: "shard-manifest-readable",
                passed: true,
                note: "LATTICE reported the bounded shard manifest as readable",
            },
            LatticeShardValidationDisplayCheck {
                name: "shard-receipt-readable",
                passed: true,
                note: "LATTICE reported the shard materialization receipt as readable",
            },
            LatticeShardValidationDisplayCheck {
                name: "aggregate-hash-replay",
                passed: true,
                note: "LATTICE replayed the aggregate hash against the fixed shard contract",
            },
            LatticeShardValidationDisplayCheck {
                name: "summary-counts-replay",
                passed: true,
                note: "LATTICE replayed shard and boundary counts from manifest and receipt",
            },
            LatticeShardValidationDisplayCheck {
                name: "fixed-middle-boundary",
                passed: true,
                note: "LATTICE preserved scans_raw_grains:false and global_closure_claimed:false",
            },
        ],
    }
}

pub fn lattice_algebra_validation_fixture() -> LatticeAlgebraValidationReport {
    LatticeAlgebraValidationReport {
        suite: "WITNESS-lattice-algebra-validation-v1",
        consumed_command: "lattice-cli validate algebra --json",
        consumed_contract_schema: "lattice.algebra-property-validation.v1",
        semantic_owner: "LATTICE",
        viewer_role: "property-proof display",
        boundary: "WITNESS displays the LATTICE algebra validation report; LATTICE owns meet/join execution and property evaluation.",
        passed: true,
        rows: vec![
            LatticeAlgebraPropertyRow {
                id: "meet_commutative",
                law: "commutative",
                operator: "meet",
                left_grain_count: 1,
                right_grain_count: 1,
                passed: true,
            },
            LatticeAlgebraPropertyRow {
                id: "join_commutative",
                law: "commutative",
                operator: "join",
                left_grain_count: 4,
                right_grain_count: 4,
                passed: true,
            },
            LatticeAlgebraPropertyRow {
                id: "meet_idempotent",
                law: "idempotent",
                operator: "meet",
                left_grain_count: 3,
                right_grain_count: 3,
                passed: true,
            },
            LatticeAlgebraPropertyRow {
                id: "join_idempotent",
                law: "idempotent",
                operator: "join",
                left_grain_count: 3,
                right_grain_count: 3,
                passed: true,
            },
            LatticeAlgebraPropertyRow {
                id: "meet_absorption",
                law: "absorption",
                operator: "meet",
                left_grain_count: 3,
                right_grain_count: 3,
                passed: true,
            },
            LatticeAlgebraPropertyRow {
                id: "join_absorption",
                law: "absorption",
                operator: "join",
                left_grain_count: 3,
                right_grain_count: 3,
                passed: true,
            },
            LatticeAlgebraPropertyRow {
                id: "meet_associative",
                law: "associative",
                operator: "meet",
                left_grain_count: 1,
                right_grain_count: 1,
                passed: true,
            },
            LatticeAlgebraPropertyRow {
                id: "join_associative",
                law: "associative",
                operator: "join",
                left_grain_count: 5,
                right_grain_count: 5,
                passed: true,
            },
            LatticeAlgebraPropertyRow {
                id: "meet_lower_bound_left",
                law: "lower_bound",
                operator: "meet",
                left_grain_count: 1,
                right_grain_count: 3,
                passed: true,
            },
            LatticeAlgebraPropertyRow {
                id: "meet_lower_bound_right",
                law: "lower_bound",
                operator: "meet",
                left_grain_count: 1,
                right_grain_count: 2,
                passed: true,
            },
            LatticeAlgebraPropertyRow {
                id: "join_upper_bound_left",
                law: "upper_bound",
                operator: "join",
                left_grain_count: 3,
                right_grain_count: 4,
                passed: true,
            },
            LatticeAlgebraPropertyRow {
                id: "join_upper_bound_right",
                law: "upper_bound",
                operator: "join",
                left_grain_count: 2,
                right_grain_count: 4,
                passed: true,
            },
            LatticeAlgebraPropertyRow {
                id: "meet_monotone_left",
                law: "monotone",
                operator: "meet",
                left_grain_count: 1,
                right_grain_count: 1,
                passed: true,
            },
            LatticeAlgebraPropertyRow {
                id: "meet_monotone_right",
                law: "monotone",
                operator: "meet",
                left_grain_count: 1,
                right_grain_count: 1,
                passed: true,
            },
            LatticeAlgebraPropertyRow {
                id: "join_monotone_left",
                law: "monotone",
                operator: "join",
                left_grain_count: 2,
                right_grain_count: 4,
                passed: true,
            },
            LatticeAlgebraPropertyRow {
                id: "join_monotone_right",
                law: "monotone",
                operator: "join",
                left_grain_count: 2,
                right_grain_count: 3,
                passed: true,
            },
            LatticeAlgebraPropertyRow {
                id: "meet_closure_witness",
                law: "closure_witness",
                operator: "meet",
                left_grain_count: 1,
                right_grain_count: 1,
                passed: true,
            },
            LatticeAlgebraPropertyRow {
                id: "join_closure_witness",
                law: "closure_witness",
                operator: "join",
                left_grain_count: 4,
                right_grain_count: 1,
                passed: true,
            },
            LatticeAlgebraPropertyRow {
                id: "meet_policy_witness",
                law: "policy_witness",
                operator: "meet",
                left_grain_count: 1,
                right_grain_count: 1,
                passed: true,
            },
            LatticeAlgebraPropertyRow {
                id: "join_policy_witness",
                law: "policy_witness",
                operator: "join",
                left_grain_count: 4,
                right_grain_count: 1,
                passed: true,
            },
            LatticeAlgebraPropertyRow {
                id: "meet_rights_witness",
                law: "rights_witness",
                operator: "meet",
                left_grain_count: 1,
                right_grain_count: 1,
                passed: true,
            },
            LatticeAlgebraPropertyRow {
                id: "join_rights_witness",
                law: "rights_witness",
                operator: "join",
                left_grain_count: 4,
                right_grain_count: 1,
                passed: true,
            },
            LatticeAlgebraPropertyRow {
                id: "meet_bond_endpoint_witness",
                law: "bond_endpoint_witness",
                operator: "meet",
                left_grain_count: 2,
                right_grain_count: 1,
                passed: true,
            },
            LatticeAlgebraPropertyRow {
                id: "join_bond_endpoint_witness",
                law: "bond_endpoint_witness",
                operator: "join",
                left_grain_count: 4,
                right_grain_count: 3,
                passed: true,
            },
            LatticeAlgebraPropertyRow {
                id: "meet_deterministic_replay",
                law: "deterministic_replay",
                operator: "meet",
                left_grain_count: 1,
                right_grain_count: 1,
                passed: true,
            },
            LatticeAlgebraPropertyRow {
                id: "join_deterministic_replay",
                law: "deterministic_replay",
                operator: "join",
                left_grain_count: 4,
                right_grain_count: 4,
                passed: true,
            },
            LatticeAlgebraPropertyRow {
                id: "meet_input_closure_witness",
                law: "input_closure_witness",
                operator: "meet",
                left_grain_count: 1,
                right_grain_count: 1,
                passed: true,
            },
            LatticeAlgebraPropertyRow {
                id: "join_input_closure_witness",
                law: "input_closure_witness",
                operator: "join",
                left_grain_count: 1,
                right_grain_count: 1,
                passed: true,
            },
            LatticeAlgebraPropertyRow {
                id: "meet_grain_set_witness",
                law: "grain_set_witness",
                operator: "meet",
                left_grain_count: 1,
                right_grain_count: 1,
                passed: true,
            },
            LatticeAlgebraPropertyRow {
                id: "join_grain_set_witness",
                law: "grain_set_witness",
                operator: "join",
                left_grain_count: 4,
                right_grain_count: 4,
                passed: true,
            },
            LatticeAlgebraPropertyRow {
                id: "meet_bond_set_witness",
                law: "bond_set_witness",
                operator: "meet",
                left_grain_count: 0,
                right_grain_count: 0,
                passed: true,
            },
            LatticeAlgebraPropertyRow {
                id: "join_bond_set_witness",
                law: "bond_set_witness",
                operator: "join",
                left_grain_count: 2,
                right_grain_count: 2,
                passed: true,
            },
            LatticeAlgebraPropertyRow {
                id: "meet_receipt_note_witness",
                law: "receipt_note_witness",
                operator: "meet",
                left_grain_count: 1,
                right_grain_count: 1,
                passed: true,
            },
            LatticeAlgebraPropertyRow {
                id: "join_receipt_note_witness",
                law: "receipt_note_witness",
                operator: "join",
                left_grain_count: 1,
                right_grain_count: 1,
                passed: true,
            },
            LatticeAlgebraPropertyRow {
                id: "meet_receipt_hash_witness",
                law: "receipt_hash_witness",
                operator: "meet",
                left_grain_count: 1,
                right_grain_count: 34,
                passed: true,
            },
            LatticeAlgebraPropertyRow {
                id: "join_receipt_hash_witness",
                law: "receipt_hash_witness",
                operator: "join",
                left_grain_count: 1,
                right_grain_count: 34,
                passed: true,
            },
            LatticeAlgebraPropertyRow {
                id: "meet_result_id_witness",
                law: "result_id_witness",
                operator: "meet",
                left_grain_count: 17,
                right_grain_count: 17,
                passed: true,
            },
            LatticeAlgebraPropertyRow {
                id: "join_result_id_witness",
                law: "result_id_witness",
                operator: "join",
                left_grain_count: 17,
                right_grain_count: 17,
                passed: true,
            },
            LatticeAlgebraPropertyRow {
                id: "meet_receipt_count_witness",
                law: "receipt_count_witness",
                operator: "meet",
                left_grain_count: 1,
                right_grain_count: 1,
                passed: true,
            },
            LatticeAlgebraPropertyRow {
                id: "join_receipt_count_witness",
                law: "receipt_count_witness",
                operator: "join",
                left_grain_count: 1,
                right_grain_count: 1,
                passed: true,
            },
            LatticeAlgebraPropertyRow {
                id: "meet_policy_value_witness",
                law: "policy_value_witness",
                operator: "meet",
                left_grain_count: 10,
                right_grain_count: 10,
                passed: true,
            },
            LatticeAlgebraPropertyRow {
                id: "join_policy_value_witness",
                law: "policy_value_witness",
                operator: "join",
                left_grain_count: 10,
                right_grain_count: 10,
                passed: true,
            },
            LatticeAlgebraPropertyRow {
                id: "meet_rights_value_witness",
                law: "rights_value_witness",
                operator: "meet",
                left_grain_count: 20,
                right_grain_count: 20,
                passed: true,
            },
            LatticeAlgebraPropertyRow {
                id: "join_rights_value_witness",
                law: "rights_value_witness",
                operator: "join",
                left_grain_count: 20,
                right_grain_count: 20,
                passed: true,
            },
            LatticeAlgebraPropertyRow {
                id: "meet_receipt_rule_witness",
                law: "receipt_rule_witness",
                operator: "meet",
                left_grain_count: 1,
                right_grain_count: 4,
                passed: true,
            },
            LatticeAlgebraPropertyRow {
                id: "join_receipt_rule_witness",
                law: "receipt_rule_witness",
                operator: "join",
                left_grain_count: 1,
                right_grain_count: 4,
                passed: true,
            },
        ],
    }
}

pub fn format_matrix_fixture() -> FormatMatrixReport {
    FormatMatrixReport {
        suite: "WITNESS-customer-format-matrix-v1",
        purpose: "Model current harness customer needs and validate LATTICE contracts without provider invocation.",
        rows: vec![
            FormatMatrixRow {
                target: "claude-code",
                customer_need: "long-running terminal agent with tool gates and resumable context",
                native_affordance: "chat-plus-tools loop with permission prompts and command execution",
                witness_model: "harness lifecycle events with permission gates, receipts, checkpoint, and rehydrate",
                lattice_validation: "source pointers, active cut, validation receipt, frontier, and rehydrated cut",
                projection_fidelity: 92,
            },
            FormatMatrixRow {
                target: "codex-style-agent",
                customer_need: "patch-oriented coding flow with tests and clear handoff of modified files",
                native_affordance: "edit plan, patch application, command validation, and diff summary",
                witness_model: "file-read, file-edit, validation-owed, checkpoint, and handoff receipts",
                lattice_validation: "transition grain, receipt-backed validation, and frontier for unrun checks",
                projection_fidelity: 88,
            },
            FormatMatrixRow {
                target: "copilot-agent",
                customer_need: "repository-aware task execution that can connect issues, PRs, and checks",
                native_affordance: "repo graph, pull request workflow, workflow status, and review comments",
                witness_model: "source-pointer candidates plus collaboration and validation scenario families",
                lattice_validation: "join source cuts, preserve provenance, and gate downstream evidence",
                projection_fidelity: 84,
            },
            FormatMatrixRow {
                target: "cursor-style-workspace",
                customer_need: "IDE-local context navigation with fast edits and visible file grounding",
                native_affordance: "workspace index, editor selection, inline edits, and local diagnostics",
                witness_model: "file-navigation and edit-cycle scenarios with source custody gates",
                lattice_validation: "pointer-only evidence, narrowed context, and source-grounded joins",
                projection_fidelity: 82,
            },
            FormatMatrixRow {
                target: "local-provider-runtime",
                customer_need: "private execution where prompts, source, and credentials remain locally controlled",
                native_affordance: "local model/runtime configuration with restricted network and file access",
                witness_model: "session safety gates, provider-neutral deltas, and explicit frontier records",
                lattice_validation: "privacy frontiers, custody receipts, and deferred closure until authorized",
                projection_fidelity: 86,
            },
        ],
    }
}

pub fn checkpoint_artifact_fixture() -> CheckpointArtifact {
    let replay = claude_session_fixture();
    CheckpointArtifact {
        fixture: "claude-session",
        checkpoint_id: "checkpoint:WITNESS:claude-session:001",
        active_cut: replay.active_cut,
        rehydrated_cut: replay.rehydrated_cut,
        receipt_count: replay.receipt_count(),
        frontier_count: replay.frontier_count,
        source_pointer_count: replay.source_pointer_count(),
        custody: "local-artifact-pointer-only",
    }
}

pub fn checkpoint_write_report(path: &str) -> CheckpointWriteReport {
    checkpoint_artifact_fixture().write_report(path)
}

pub fn session_corpus_review_fixture() -> SessionCorpusReviewReport {
    let gates = vec![
        SessionCorpusGate {
            id: "explicit-consent",
            requirement: "User or workspace owner explicitly authorizes the session corpus scope.",
            status: "required-before-ingest",
            lattice_validation: "consent receipt must gate any source pointer.",
        },
        SessionCorpusGate {
            id: "pointer-first-custody",
            requirement: "WITNESS records session pointers, hashes, and summaries before copying content.",
            status: "required-before-ingest",
            lattice_validation: "source pointer and custody receipt must precede closure.",
        },
        SessionCorpusGate {
            id: "secret-exclusion",
            requirement: "Credentials, tokens, private keys, and sensitive local files are excluded.",
            status: "required-before-ingest",
            lattice_validation: "excluded material remains frontiered, never closed.",
        },
        SessionCorpusGate {
            id: "raw-chat-minimization",
            requirement: "Raw chat logs are treated as sensitive source material, not the source of truth.",
            status: "required-before-ingest",
            lattice_validation: "summaries and hashes validate replay without opaque blob dependence.",
        },
        SessionCorpusGate {
            id: "retention-boundary",
            requirement: "Review artifacts state where data is stored, for how long, and whether it is committed.",
            status: "required-before-ingest",
            lattice_validation: "retention receipt determines whether a cut can close or stays frontiered.",
        },
        SessionCorpusGate {
            id: "provider-boundary",
            requirement: "No session corpus content is sent to a provider in the foundation wave.",
            status: "required-before-ingest",
            lattice_validation: "provider use remains a frontier until adapter review.",
        },
    ];
    SessionCorpusReviewReport {
        suite: "WITNESS-session-corpus-review-v1",
        purpose: "Define safe local session-corpus intake boundaries before real chat analysis.",
        raw_ingestion_allowed: false,
        gate_count: gates.len(),
        approved_gate_count: gates.len(),
        gates,
    }
}

pub fn maxim_scenario_fixture() -> MaximScenarioReport {
    let scenarios = vec![
        MaximScenario {
            id: "maxim-001",
            user_goal: "Decide whether a product launch is ready.",
            source_pointer: "maxim:product-launch/readiness-signals",
            receipt: "receipt:maxim-product-launch-readiness",
            lattice_validation:
                "join market, customer, risk, timing, and readiness cuts before handoff.",
            result_shape: "launch-craft",
            saved_chunk: "chunk:launch-readiness-pack",
        },
        MaximScenario {
            id: "maxim-002",
            user_goal: "Compare competing product narratives.",
            source_pointer: "maxim:product-launch/narrative-options",
            receipt: "receipt:maxim-product-narratives",
            lattice_validation:
                "narrow to narratives with cited source pointers and frontier unsupported claims.",
            result_shape: "fan-out-fan-in",
            saved_chunk: "chunk:narrative-comparison-pack",
        },
        MaximScenario {
            id: "maxim-003",
            user_goal: "Synthesize research evidence for a decision memo.",
            source_pointer: "maxim:research/evidence-map",
            receipt: "receipt:maxim-research-evidence",
            lattice_validation:
                "convert evidence pointers into a cited constellation before closure.",
            result_shape: "evidence-constellation",
            saved_chunk: "chunk:cited-evidence-map",
        },
        MaximScenario {
            id: "maxim-004",
            user_goal: "Plan a source-grounded implementation path.",
            source_pointer: "maxim:implementation/path-options",
            receipt: "receipt:maxim-implementation-path",
            lattice_validation:
                "route candidate cuts through validation, dependency, and frontier gates.",
            result_shape: "route-lattice",
            saved_chunk: "chunk:implementation-route",
        },
        MaximScenario {
            id: "maxim-005",
            user_goal: "Review policy or compliance constraints.",
            source_pointer: "maxim:policy/constraint-set",
            receipt: "receipt:maxim-policy-constraints",
            lattice_validation:
                "meet active intent with constraint cuts and preserve exclusions as frontier.",
            result_shape: "constraint-enclosure",
            saved_chunk: "chunk:policy-constraint-pack",
        },
        MaximScenario {
            id: "maxim-006",
            user_goal: "Build a reusable source pack for later work.",
            source_pointer: "maxim:source-pack/reusable-context",
            receipt: "receipt:maxim-reusable-source-pack",
            lattice_validation:
                "save pointer-only chunks as packs that LATTICE can later close or reject.",
            result_shape: "cargo-crates",
            saved_chunk: "chunk:reusable-source-cargo",
        },
    ];
    MaximScenarioReport {
        suite: "WITNESS-maxim-source-scenarios-v1",
        corpus: "MAXIM",
        custody: "pointer-only",
        scenario_count: scenarios.len(),
        pointer_count: scenarios.len(),
        receipt_count: scenarios.len(),
        frontier_count: 2,
        scenarios,
    }
}

pub fn maxim_conversation_fixture() -> MaximConversationReport {
    let conversations = vec![
        MaximConversation {
            id: "maxim-conv-001",
            title: "Launch readiness conversation",
            user_goal: "Decide whether a product launch is ready using MAXIM-style reference grounding.",
            source_pointer: "maxim:product-launch/readiness-signals",
            receipt: "receipt:maxim-product-launch-readiness",
            expected_shape: "launch-craft",
            evaluation_gate: "passed:pointer-receipt-frontier-visual",
            turns: vec![
                MaximConversationTurn {
                    id: "maxim-conv-001:t1",
                    speaker: "user",
                    summary: "asks for a launch readiness decision",
                    harness_event: "intent-captured",
                    visual_update: "wide launch context field opens",
                    lattice_anchor: "frontier:world-context",
                },
                MaximConversationTurn {
                    id: "maxim-conv-001:t2",
                    speaker: "assistant",
                    summary: "requests MAXIM readiness, customer, risk, timing, and validation pointers",
                    harness_event: "source-scope-requested",
                    visual_update: "five launch craft regions highlight",
                    lattice_anchor: "maxim:product-launch/readiness-signals",
                },
                MaximConversationTurn {
                    id: "maxim-conv-001:t3",
                    speaker: "user",
                    summary: "adds MAXIM readiness and policy source pointers",
                    harness_event: "source-pointers-added",
                    visual_update: "evidence snaps into the launch craft",
                    lattice_anchor: "receipt:maxim-product-launch-readiness",
                },
                MaximConversationTurn {
                    id: "maxim-conv-001:t4",
                    speaker: "assistant",
                    summary: "returns a launch recommendation with execution frontier visible",
                    harness_event: "goal-artifact-projected",
                    visual_update: "launch craft forms with frontier cargo attached",
                    lattice_anchor: "shape:launch-craft",
                },
            ],
        },
        MaximConversation {
            id: "maxim-conv-002",
            title: "Research evidence conversation",
            user_goal: "Build a cited decision memo from MAXIM research evidence.",
            source_pointer: "maxim:research/evidence-map",
            receipt: "receipt:maxim-research-evidence",
            expected_shape: "evidence-constellation",
            evaluation_gate: "passed:pointer-receipt-frontier-visual",
            turns: vec![
                MaximConversationTurn {
                    id: "maxim-conv-002:t1",
                    speaker: "user",
                    summary: "asks for a source-grounded evidence memo",
                    harness_event: "intent-captured",
                    visual_update: "research field opens as unresolved constellation",
                    lattice_anchor: "frontier:research-scope",
                },
                MaximConversationTurn {
                    id: "maxim-conv-002:t2",
                    speaker: "assistant",
                    summary: "narrows to MAXIM evidence pointers and asks what decision the memo supports",
                    harness_event: "context-narrowed",
                    visual_update: "candidate stars dim outside the decision scope",
                    lattice_anchor: "maxim:research/evidence-map",
                },
                MaximConversationTurn {
                    id: "maxim-conv-002:t3",
                    speaker: "user",
                    summary: "confirms the decision and acceptable evidence boundary",
                    harness_event: "receipts-joined",
                    visual_update: "cited evidence nodes connect into a constellation",
                    lattice_anchor: "receipt:maxim-research-evidence",
                },
                MaximConversationTurn {
                    id: "maxim-conv-002:t4",
                    speaker: "assistant",
                    summary: "returns memo structure and saves a cited evidence map",
                    harness_event: "chunk-saved",
                    visual_update: "evidence map crystallizes as reusable context",
                    lattice_anchor: "chunk:cited-evidence-map",
                },
            ],
        },
        MaximConversation {
            id: "maxim-conv-003",
            title: "Implementation route conversation",
            user_goal: "Plan a source-grounded implementation path with unresolved route work preserved.",
            source_pointer: "maxim:implementation/path-options",
            receipt: "receipt:maxim-implementation-path",
            expected_shape: "route-lattice",
            evaluation_gate: "passed:pointer-receipt-frontier-visual",
            turns: vec![
                MaximConversationTurn {
                    id: "maxim-conv-003:t1",
                    speaker: "user",
                    summary: "asks for an implementation route",
                    harness_event: "intent-captured",
                    visual_update: "route lattice opens with candidate paths",
                    lattice_anchor: "frontier:route-options",
                },
                MaximConversationTurn {
                    id: "maxim-conv-003:t2",
                    speaker: "assistant",
                    summary: "requests dependency, validation, and source constraints",
                    harness_event: "source-scope-requested",
                    visual_update: "dependency gates appear along candidate routes",
                    lattice_anchor: "maxim:implementation/path-options",
                },
                MaximConversationTurn {
                    id: "maxim-conv-003:t3",
                    speaker: "user",
                    summary: "selects the implementation constraints and validation expectations",
                    harness_event: "constraints-applied",
                    visual_update: "invalid paths gray into frontier",
                    lattice_anchor: "receipt:maxim-implementation-path",
                },
                MaximConversationTurn {
                    id: "maxim-conv-003:t4",
                    speaker: "assistant",
                    summary: "returns the route and keeps validation debt visible",
                    harness_event: "frontier-preserved",
                    visual_update: "chosen route brightens while validation frontier remains attached",
                    lattice_anchor: "frontier:lattice-execution",
                },
            ],
        },
    ];
    MaximConversationReport {
        suite: "WITNESS-maxim-conversation-scenarios-v1",
        corpus: "MAXIM",
        provider_mode: "deterministic-no-provider-call",
        custody: "pointer-only",
        semantic_owner: "LATTICE",
        conversation_count: conversations.len(),
        turn_count: conversations
            .iter()
            .map(|conversation| conversation.turns.len())
            .sum(),
        pointer_count: conversations.len(),
        receipt_count: conversations.len(),
        visual_update_count: conversations
            .iter()
            .map(|conversation| conversation.turns.len())
            .sum(),
        passed_gate_count: conversations
            .iter()
            .filter(|conversation| conversation.evaluation_gate.starts_with("passed:"))
            .count(),
        conversations,
    }
}

pub fn maxim_conversation_eval_fixture() -> MaximConversationEvaluationReport {
    let conversations = maxim_conversation_fixture();
    let results = vec![
        MaximConversationEvaluation {
            id: "maxim-conv-001",
            title: "Launch readiness conversation",
            pointer_gate: "passed:maxim-pointer-present",
            receipt_gate: "passed:receipt-present",
            visual_gate: "passed:visual-update-per-turn",
            frontier_gate: "passed:frontier-visible",
            shape_gate: "passed:launch-craft",
            result: "passed",
        },
        MaximConversationEvaluation {
            id: "maxim-conv-002",
            title: "Research evidence conversation",
            pointer_gate: "passed:maxim-pointer-present",
            receipt_gate: "passed:receipt-present",
            visual_gate: "passed:visual-update-per-turn",
            frontier_gate: "passed:frontier-visible",
            shape_gate: "passed:evidence-constellation",
            result: "passed",
        },
        MaximConversationEvaluation {
            id: "maxim-conv-003",
            title: "Implementation route conversation",
            pointer_gate: "passed:maxim-pointer-present",
            receipt_gate: "passed:receipt-present",
            visual_gate: "passed:visual-update-per-turn",
            frontier_gate: "passed:frontier-visible",
            shape_gate: "passed:route-lattice",
            result: "passed",
        },
    ];
    MaximConversationEvaluationReport {
        suite: "WITNESS-maxim-conversation-eval-v1",
        corpus: "MAXIM",
        evaluation_target: "witness.maxim-conversations.v1",
        semantic_owner: "LATTICE",
        scenario_count: conversations.conversation_count,
        passed_count: results
            .iter()
            .filter(|evaluation| evaluation.result == "passed")
            .count(),
        failed_count: results
            .iter()
            .filter(|evaluation| evaluation.result != "passed")
            .count(),
        gate_count: results.len() * 5,
        results,
    }
}

pub fn maxim_conversation_replay_fixture() -> MaximConversationReplayReport {
    let frames = vec![
        MaximConversationReplayFrame {
            turn_id: "replay:launch:t1",
            speaker: "user",
            message: "Can we launch this product next month, and what context do you need?",
            expected_delta: "keep_intent",
            visual_frame: "world-field-opened",
            lattice_anchor: "frontier:world-context",
            assertion: "chat remains primary while visualization opens unresolved context",
        },
        MaximConversationReplayFrame {
            turn_id: "replay:launch:t2",
            speaker: "assistant",
            message: "I need readiness, customer proof, risk constraints, timing, and validation pointers.",
            expected_delta: "frontier_gap",
            visual_frame: "focus-cone-formed",
            lattice_anchor: "frontier:missing-launch-criteria",
            assertion: "assistant asks for source context instead of hallucinating closure",
        },
        MaximConversationReplayFrame {
            turn_id: "replay:launch:t3",
            speaker: "user",
            message: "Use MAXIM launch readiness, policy constraints, and implementation path pointers.",
            expected_delta: "add_source",
            visual_frame: "source-panels-snapped-in",
            lattice_anchor: "maxim:product-launch/readiness-signals",
            assertion: "MAXIM enters as a pointer, not copied source payload",
        },
        MaximConversationReplayFrame {
            turn_id: "replay:launch:t4",
            speaker: "assistant",
            message: "Market and customer evidence support launch, but policy risk constrains rollout.",
            expected_delta: "join_context",
            visual_frame: "evidence-joined-with-constraint-shield",
            lattice_anchor: "receipt:maxim-product-launch-readiness",
            assertion: "evidence and constraints join through receipts",
        },
        MaximConversationReplayFrame {
            turn_id: "replay:launch:t5",
            speaker: "assistant",
            message: "Save the launch readiness pack and keep execution route validation as frontier.",
            expected_delta: "checkpoint",
            visual_frame: "chunk-crystallized-frontier-attached",
            lattice_anchor: "chunk:launch-readiness-pack",
            assertion: "saved chunk is visible and unresolved work remains frontier",
        },
        MaximConversationReplayFrame {
            turn_id: "replay:launch:t6",
            speaker: "assistant",
            message: "Recommendation: launch with a staged rollout once route validation clears.",
            expected_delta: "lattice_operation",
            visual_frame: "launch-craft-formed-with-frontier-cargo",
            lattice_anchor: "shape:launch-craft",
            assertion: "goal artifact forms without hiding remaining frontier",
        },
    ];
    MaximConversationReplayReport {
        suite: "WITNESS-maxim-conversation-replay-v1",
        transcript: "maxim-launch-readiness-chat",
        corpus: "MAXIM",
        semantic_owner: "LATTICE",
        frame_count: frames.len(),
        visual_frame_count: frames.len(),
        source_anchor_count: frames
            .iter()
            .filter(|frame| frame.lattice_anchor.starts_with("maxim:"))
            .count(),
        frontier_frame_count: frames
            .iter()
            .filter(|frame| frame.lattice_anchor.starts_with("frontier:"))
            .count(),
        frames,
    }
}

pub fn maxim_conversation_sync_fixture() -> MaximConversationSyncReport {
    let checks = vec![
        MaximConversationSyncCheck {
            id: "sync:chat-primary",
            replay_frame: "replay:launch:t1",
            chat_surface_state: "user intent visible in chat",
            visual_contract: "witness.conversation-surface.v1",
            expected_artifact: "chat-with-companion-visual",
            sync_gate: "passed:chat-primary",
            result: "passed",
        },
        MaximConversationSyncCheck {
            id: "sync:world-field",
            replay_frame: "replay:launch:t1",
            chat_surface_state: "intent captured",
            visual_contract: "witness.visual-context.v1",
            expected_artifact: "world-field",
            sync_gate: "passed:visual-opened-on-intent",
            result: "passed",
        },
        MaximConversationSyncCheck {
            id: "sync:maxim-pointer",
            replay_frame: "replay:launch:t3",
            chat_surface_state: "MAXIM pointers added by user",
            visual_contract: "witness.maxim-conversation-replay.v1",
            expected_artifact: "source-panels-snapped-in",
            sync_gate: "passed:pointer-to-visual-frame",
            result: "passed",
        },
        MaximConversationSyncCheck {
            id: "sync:receipt-join",
            replay_frame: "replay:launch:t4",
            chat_surface_state: "assistant joins readiness evidence with risk constraints",
            visual_contract: "witness.build-meter.v1",
            expected_artifact: "evidence-joined-with-constraint-shield",
            sync_gate: "passed:receipt-to-meter-stage",
            result: "passed",
        },
        MaximConversationSyncCheck {
            id: "sync:saved-chunk",
            replay_frame: "replay:launch:t5",
            chat_surface_state: "assistant saves launch readiness pack",
            visual_contract: "witness.chunk-inventory.v1",
            expected_artifact: "chunk-crystallized-frontier-attached",
            sync_gate: "passed:chunk-visible",
            result: "passed",
        },
        MaximConversationSyncCheck {
            id: "sync:goal-artifact",
            replay_frame: "replay:launch:t6",
            chat_surface_state: "assistant presents staged-launch recommendation",
            visual_contract: "witness.artifact-assembly.v1",
            expected_artifact: "launch-craft-with-frontier-cargo",
            sync_gate: "passed:artifact-with-frontier",
            result: "passed",
        },
    ];
    MaximConversationSyncReport {
        suite: "WITNESS-maxim-conversation-sync-v1",
        transcript: "maxim-launch-readiness-chat",
        sync_target: "chat-surface-plus-context-visualization",
        semantic_owner: "LATTICE",
        check_count: checks.len(),
        passed_count: checks
            .iter()
            .filter(|check| check.result == "passed")
            .count(),
        failed_count: checks
            .iter()
            .filter(|check| check.result != "passed")
            .count(),
        checks,
    }
}

pub fn chat_web_readiness_fixture() -> ChatWebReadinessReport {
    let routes = vec![
        ChatWebRoute {
            path: "/",
            surface: "chat-shell",
            data_feed: "witness.conversation-surface.v1",
            render_state: "two-pane chat plus context visualization",
            privacy_gate: "local-only",
            ready_gate: "ready",
        },
        ChatWebRoute {
            path: "/api/chat/replay",
            surface: "chat-transcript-feed",
            data_feed: "witness.maxim-conversation-replay.v1",
            render_state: "turn list with expected deltas",
            privacy_gate: "fixture-only",
            ready_gate: "ready",
        },
        ChatWebRoute {
            path: "/api/chat/sync",
            surface: "chat-visual-sync-feed",
            data_feed: "witness.maxim-conversation-sync.v1",
            render_state: "lockstep chat and visual assertions",
            privacy_gate: "no-raw-source",
            ready_gate: "ready",
        },
        ChatWebRoute {
            path: "/api/visuals/meter",
            surface: "progress-meter",
            data_feed: "witness.build-meter.v1",
            render_state: "goal focus through artifact formation",
            privacy_gate: "pointer-only",
            ready_gate: "ready",
        },
        ChatWebRoute {
            path: "/api/visuals/assembly",
            surface: "goal-artifact",
            data_feed: "witness.artifact-assembly.v1",
            render_state: "launch craft with frontier cargo",
            privacy_gate: "pointer-only",
            ready_gate: "ready",
        },
        ChatWebRoute {
            path: "/api/visuals/narrowing",
            surface: "query-narrowing",
            data_feed: "witness.narrowing-trace.v1",
            render_state: "world field constrained by user questions",
            privacy_gate: "pointer-only",
            ready_gate: "ready",
        },
    ];
    ChatWebReadinessReport {
        suite: "WITNESS-chat-web-readiness-v1",
        intended_command: "witness-cli chat web --port 8787",
        host_policy: "localhost-only-fixture-backed",
        semantic_owner: "LATTICE",
        route_count: routes.len(),
        data_feed_count: routes.len(),
        privacy_gate_count: routes.len(),
        ready_gate_count: routes
            .iter()
            .filter(|route| route.ready_gate == "ready")
            .count(),
        routes,
    }
}

pub fn chat_web_fixture_bundle() -> ChatWebFixtureBundleReport {
    ChatWebFixtureBundleReport {
        suite: "WITNESS-chat-web-fixture-v1",
        bootstrap_route: "/api/bootstrap",
        host_policy: "localhost-only-fixture-backed",
        semantic_owner: "LATTICE",
        feed_count: 7,
        privacy_gate: "pointer-only-no-provider-no-raw-source",
    }
}

pub fn chat_web_storyboard_fixture() -> ChatWebStoryboardReport {
    let panels = vec![
        ChatWebPanel {
            id: "panel:chat",
            title: "Conversation",
            role: "primary harness chat",
            data_feed: "witness.maxim-conversation-replay.v1",
            render_contract: "turns remain readable while visuals update beside them",
        },
        ChatWebPanel {
            id: "panel:context-field",
            title: "Context field",
            role: "world context narrowing visualization",
            data_feed: "witness.narrowing-trace.v1",
            render_contract: "candidate field narrows after user questions",
        },
        ChatWebPanel {
            id: "panel:meter",
            title: "Build meter",
            role: "visible progress and satisfaction loop",
            data_feed: "witness.build-meter.v1",
            render_contract: "goal focus, evidence join, constraints, chunks, frontier, artifact",
        },
        ChatWebPanel {
            id: "panel:artifact",
            title: "Launch craft",
            role: "scenario-native goal artifact",
            data_feed: "witness.artifact-assembly.v1",
            render_contract: "included pieces and frontier cargo stay visible",
        },
        ChatWebPanel {
            id: "panel:frontier",
            title: "Frontier",
            role: "unresolved work and caveats",
            data_feed: "witness.maxim-conversation-sync.v1",
            render_contract: "unresolved context is visible, not hidden as completion",
        },
    ];
    let frames = vec![
        ChatWebStoryboardFrame {
            id: "story:open-field",
            replay_frame: "replay:launch:t1",
            active_panel: "panel:context-field",
            animation_cue: "dim candidate field expands behind the chat",
            user_feedback: "I can see the system opening the possible context space.",
            frontier_state: "frontier:world-context",
        },
        ChatWebStoryboardFrame {
            id: "story:focus-cone",
            replay_frame: "replay:launch:t2",
            active_panel: "panel:meter",
            animation_cue: "focus cone locks onto launch readiness dimensions",
            user_feedback: "The assistant is asking for missing context instead of guessing.",
            frontier_state: "frontier:missing-launch-criteria",
        },
        ChatWebStoryboardFrame {
            id: "story:sources-snap",
            replay_frame: "replay:launch:t3",
            active_panel: "panel:context-field",
            animation_cue: "MAXIM source panels snap into the narrowing field",
            user_feedback: "My referenced sources became visible without exposing raw content.",
            frontier_state: "frontier:weak-customer-evidence",
        },
        ChatWebStoryboardFrame {
            id: "story:constraint-shield",
            replay_frame: "replay:launch:t4",
            active_panel: "panel:meter",
            animation_cue: "risk constraints wrap evidence as a shield",
            user_feedback: "The visual explains why the recommendation is constrained.",
            frontier_state: "frontier:waiver-review",
        },
        ChatWebStoryboardFrame {
            id: "story:chunk-crystal",
            replay_frame: "replay:launch:t5",
            active_panel: "panel:frontier",
            animation_cue: "launch readiness pack crystallizes while route cargo remains attached",
            user_feedback: "Useful context was saved and the remaining work stayed visible.",
            frontier_state: "frontier:lattice-execution",
        },
        ChatWebStoryboardFrame {
            id: "story:launch-craft",
            replay_frame: "replay:launch:t6",
            active_panel: "panel:artifact",
            animation_cue: "launch craft forms with frontier cargo still connected",
            user_feedback: "The final shape shows what is included and what is still open.",
            frontier_state: "frontier:lattice-execution",
        },
    ];
    ChatWebStoryboardReport {
        suite: "WITNESS-chat-web-storyboard-v1",
        target_surface: "chat-web-localhost",
        semantic_owner: "LATTICE",
        panel_count: panels.len(),
        frame_count: frames.len(),
        panels,
        frames,
    }
}

pub fn chat_web_acceptance_fixture() -> ChatWebAcceptanceReport {
    let criteria = vec![
        ChatWebAcceptanceCriterion {
            id: "accept:localhost",
            requirement: "web server binds only to localhost by default",
            evidence_feed: "witness.chat-web-readiness.v1",
            expected_check: "host_policy is localhost-only-fixture-backed",
            gate: "required",
        },
        ChatWebAcceptanceCriterion {
            id: "accept:bootstrap",
            requirement: "bootstrap route returns every feed needed for first render",
            evidence_feed: "witness.chat-web-fixture.v1",
            expected_check: "/api/bootstrap includes chat, replay, sync, meter, assembly, and narrowing feeds",
            gate: "required",
        },
        ChatWebAcceptanceCriterion {
            id: "accept:chat-primary",
            requirement: "conversation remains the primary harness surface",
            evidence_feed: "witness.conversation-surface.v1",
            expected_check: "primary_surface is chat and companion_surface is context-visualization",
            gate: "required",
        },
        ChatWebAcceptanceCriterion {
            id: "accept:storyboard",
            requirement: "viewer renders storyboard panels and replay activation order",
            evidence_feed: "witness.chat-web-storyboard.v1",
            expected_check: "five panels and six replay frames are available",
            gate: "required",
        },
        ChatWebAcceptanceCriterion {
            id: "accept:frontier",
            requirement: "frontier stays visible through final artifact formation",
            evidence_feed: "witness.maxim-conversation-sync.v1",
            expected_check: "artifact sync gate requires frontier cargo",
            gate: "required",
        },
        ChatWebAcceptanceCriterion {
            id: "accept:safety",
            requirement: "first viewer uses fixture data only and does not call providers or read raw source payloads",
            evidence_feed: "witness.chat-web-fixture.v1",
            expected_check: "privacy_gate is pointer-only-no-provider-no-raw-source",
            gate: "required",
        },
    ];
    ChatWebAcceptanceReport {
        suite: "WITNESS-chat-web-acceptance-v1",
        target_command: "witness-cli chat web --port 8787",
        semantic_owner: "LATTICE",
        criterion_count: criteria.len(),
        required_count: criteria
            .iter()
            .filter(|criterion| criterion.gate == "required")
            .count(),
        criteria,
    }
}

pub fn visual_context_fixture() -> VisualContextReport {
    let events = vec![
        VisualContextEvent {
            id: "visual-001",
            motion: "wide unresolved field",
            lattice_contract: "candidate source space and frontier",
            user_feedback: "The world of possible context is visible before narrowing.",
            shape: "world-field",
        },
        VisualContextEvent {
            id: "visual-002",
            motion: "bright narrowing region",
            lattice_contract: "meet active intent with launch-readiness constraints",
            user_feedback: "The query visibly focuses the model toward the launch goal.",
            shape: "focus-cone",
        },
        VisualContextEvent {
            id: "visual-003",
            motion: "evidence pieces snapping in",
            lattice_contract: "source pointers and receipts join market, customer, risk, timing, and readiness cuts",
            user_feedback: "Included evidence feels assembled, not hidden.",
            shape: "snap-in-panels",
        },
        VisualContextEvent {
            id: "visual-004",
            motion: "branches graying out",
            lattice_contract: "unsafe, irrelevant, or unsupported context moves to frontier",
            user_feedback: "Deferred context remains visible without polluting the decision.",
            shape: "frontier-shadow",
        },
        VisualContextEvent {
            id: "visual-005",
            motion: "chunks crystallizing",
            lattice_contract: "saved cuts, packs, and checkpoint artifacts",
            user_feedback: "Reusable launch-readiness chunks can be saved and carried forward.",
            shape: "cargo-crates",
        },
        VisualContextEvent {
            id: "visual-006",
            motion: "goal artifact forming",
            lattice_contract: "handoff-ready cut projected as a scenario-native result shape",
            user_feedback: "The product launch decision craft shows all required pieces included.",
            shape: "launch-craft",
        },
    ];
    VisualContextReport {
        suite: "WITNESS-visual-context-shaping-v1",
        scenario: "product-launch-decision",
        goal_artifact: "launch-craft",
        semantic_owner: "LATTICE",
        event_count: events.len(),
        saved_chunk_count: 2,
        frontier_count: 2,
        events,
    }
}

pub fn visual_shape_catalog_fixture() -> VisualShapeCatalog {
    let shapes = vec![
        VisualShape {
            id: "shape:launch-craft",
            scenario: "product launch decision",
            artifact_shape: "launch-craft",
            user_satisfaction: "all launch pieces are visibly assembled around the go/no-go goal",
            required_lattice_evidence: "market, customer, risk, timing, and readiness source cuts with receipts",
            saveable_chunk: "chunk:launch-readiness-pack",
        },
        VisualShape {
            id: "shape:evidence-constellation",
            scenario: "research synthesis",
            artifact_shape: "evidence-constellation",
            user_satisfaction: "scattered sources collapse into a cited evidence map",
            required_lattice_evidence: "source pointers, citation receipts, and unsupported-claim frontier",
            saveable_chunk: "chunk:cited-evidence-map",
        },
        VisualShape {
            id: "shape:fault-circuit",
            scenario: "debugging or repair",
            artifact_shape: "fault-circuit",
            user_satisfaction: "fault paths narrow into a repaired circuit with validation lights",
            required_lattice_evidence: "read receipts, edit transition receipts, validation receipts, and unresolved frontier",
            saveable_chunk: "chunk:repair-trace",
        },
        VisualShape {
            id: "shape:route-lattice",
            scenario: "planning or implementation path",
            artifact_shape: "route-lattice",
            user_satisfaction: "candidate paths fan out, gate, and converge into an executable route",
            required_lattice_evidence: "dependency cuts, validation gates, sequence receipts, and blocked-route frontier",
            saveable_chunk: "chunk:implementation-route",
        },
        VisualShape {
            id: "shape:constraint-enclosure",
            scenario: "policy or compliance review",
            artifact_shape: "constraint-enclosure",
            user_satisfaction: "constraints visibly surround the safe decision space",
            required_lattice_evidence: "policy source pointers, meet constraints, exclusion receipts, and waiver frontier",
            saveable_chunk: "chunk:policy-constraint-pack",
        },
        VisualShape {
            id: "shape:cargo-crates",
            scenario: "source corpus packing",
            artifact_shape: "cargo-crates",
            user_satisfaction: "reusable context chunks become carry-forward packs",
            required_lattice_evidence: "pointer-only source packs, custody receipts, saved cuts, and closure frontier",
            saveable_chunk: "chunk:reusable-source-cargo",
        },
    ];
    VisualShapeCatalog {
        suite: "WITNESS-visual-shape-catalog-v1",
        semantic_owner: "LATTICE",
        shape_count: shapes.len(),
        shapes,
    }
}

pub fn chunk_inventory_fixture() -> ChunkInventoryReport {
    let chunks = vec![
        ContextChunk {
            id: "chunk:launch-readiness-pack",
            title: "Launch readiness pack",
            source: "maxim:product-launch/readiness-signals",
            artifact_shape: "launch-craft",
            lattice_anchor: "cut:WITNESS:claude-session:after-response-deltas",
            custody: "pointer-only",
            reuse_status: "reusable",
        },
        ContextChunk {
            id: "chunk:cited-evidence-map",
            title: "Cited evidence map",
            source: "maxim:research/evidence-map",
            artifact_shape: "evidence-constellation",
            lattice_anchor: "receipt:maxim-research-evidence",
            custody: "pointer-only",
            reuse_status: "reusable",
        },
        ContextChunk {
            id: "chunk:implementation-route",
            title: "Implementation route",
            source: "maxim:implementation/path-options",
            artifact_shape: "route-lattice",
            lattice_anchor: "frontier:lattice-execution",
            custody: "pointer-only",
            reuse_status: "frontier-until-validated",
        },
        ContextChunk {
            id: "chunk:policy-constraint-pack",
            title: "Policy constraint pack",
            source: "maxim:policy/constraint-set",
            artifact_shape: "constraint-enclosure",
            lattice_anchor: "receipt:maxim-policy-constraints",
            custody: "pointer-only",
            reuse_status: "reusable",
        },
        ContextChunk {
            id: "chunk:reusable-source-cargo",
            title: "Reusable source cargo",
            source: "maxim:source-pack/reusable-context",
            artifact_shape: "cargo-crates",
            lattice_anchor: "checkpoint:WITNESS:claude-session:001",
            custody: "pointer-only",
            reuse_status: "reusable",
        },
    ];
    ChunkInventoryReport {
        suite: "WITNESS-context-chunk-inventory-v1",
        semantic_owner: "LATTICE",
        inventory_status: "user-visible-saved-context",
        chunk_count: chunks.len(),
        reusable_count: chunks
            .iter()
            .filter(|chunk| chunk.reuse_status == "reusable")
            .count(),
        frontier_count: chunks
            .iter()
            .filter(|chunk| chunk.reuse_status != "reusable")
            .count(),
        chunks,
    }
}

pub fn build_meter_fixture() -> BuildMeterReport {
    let stages = vec![
        BuildMeterStage {
            id: "meter:intent",
            label: "Goal focused",
            progress: 100,
            signal: "active user goal captured",
            lattice_anchor: "cut:WITNESS:claude-session:active",
            visual_feedback: "focus cone locks onto the product launch decision",
        },
        BuildMeterStage {
            id: "meter:evidence",
            label: "Evidence joined",
            progress: 83,
            signal: "source pointers and receipts joined",
            lattice_anchor: "receipt:maxim-product-launch-readiness",
            visual_feedback: "market, customer, risk, timing, and readiness panels snap into place",
        },
        BuildMeterStage {
            id: "meter:constraints",
            label: "Constraints applied",
            progress: 76,
            signal: "meet constraints and exclusions applied",
            lattice_anchor: "chunk:policy-constraint-pack",
            visual_feedback: "unsafe or irrelevant branches gray out into frontier",
        },
        BuildMeterStage {
            id: "meter:chunks",
            label: "Chunks saved",
            progress: 80,
            signal: "reusable chunks crystallized",
            lattice_anchor: "chunk:launch-readiness-pack",
            visual_feedback: "launch readiness and evidence chunks become visible craft materials",
        },
        BuildMeterStage {
            id: "meter:frontier",
            label: "Frontier visible",
            progress: 67,
            signal: "unresolved routes preserved",
            lattice_anchor: "frontier:lattice-execution",
            visual_feedback: "open questions stay visible instead of pretending to be complete",
        },
        BuildMeterStage {
            id: "meter:artifact",
            label: "Goal artifact formed",
            progress: 88,
            signal: "handoff-ready launch craft projected",
            lattice_anchor: "shape:launch-craft",
            visual_feedback: "the launch craft shows included pieces and remaining frontier",
        },
    ];
    BuildMeterReport {
        suite: "WITNESS-context-build-meter-v1",
        scenario: "product-launch-decision",
        semantic_owner: "LATTICE",
        overall_progress: 82,
        stage_count: stages.len(),
        stages,
    }
}

pub fn artifact_assembly_fixture() -> ArtifactAssemblyReport {
    let pieces = vec![
        ArtifactPiece {
            id: "piece:market-signal",
            label: "Market signal",
            role: "launch craft nose",
            lattice_anchor: "receipt:maxim-product-launch-readiness",
            fit_status: "included",
            visual_feedback: "market demand snaps into the front of the launch craft",
        },
        ArtifactPiece {
            id: "piece:customer-proof",
            label: "Customer proof",
            role: "launch craft cabin",
            lattice_anchor: "chunk:launch-readiness-pack",
            fit_status: "included",
            visual_feedback: "customer evidence locks into the decision cabin",
        },
        ArtifactPiece {
            id: "piece:risk-constraint",
            label: "Risk constraint",
            role: "safety shield",
            lattice_anchor: "chunk:policy-constraint-pack",
            fit_status: "included",
            visual_feedback: "risk constraints wrap the craft as a visible safety shield",
        },
        ArtifactPiece {
            id: "piece:timing-window",
            label: "Timing window",
            role: "launch trajectory",
            lattice_anchor: "cut:WITNESS:claude-session:after-response-deltas",
            fit_status: "included",
            visual_feedback: "the viable release window draws the launch path",
        },
        ArtifactPiece {
            id: "piece:readiness-check",
            label: "Readiness check",
            role: "go/no-go panel",
            lattice_anchor: "checkpoint:WITNESS:claude-session:001",
            fit_status: "included",
            visual_feedback: "validation state lights the go/no-go panel",
        },
        ArtifactPiece {
            id: "piece:execution-route",
            label: "Execution route",
            role: "frontier cargo",
            lattice_anchor: "frontier:lattice-execution",
            fit_status: "frontier",
            visual_feedback: "implementation route stays attached as unresolved cargo",
        },
    ];
    ArtifactAssemblyReport {
        suite: "WITNESS-artifact-assembly-v1",
        scenario: "product-launch-decision",
        artifact_shape: "launch-craft",
        semantic_owner: "LATTICE",
        piece_count: pieces.len(),
        included_count: pieces
            .iter()
            .filter(|piece| piece.fit_status == "included")
            .count(),
        frontier_count: pieces
            .iter()
            .filter(|piece| piece.fit_status == "frontier")
            .count(),
        pieces,
    }
}

pub fn narrowing_trace_fixture() -> NarrowingTraceReport {
    let steps = vec![
        NarrowingStep {
            id: "narrow:world-field",
            query: "What context could matter to this launch decision?",
            lattice_operation: "candidate frontier opened",
            before_count: 4096,
            after_count: 1024,
            saved_chunk: "none",
            frontier: "frontier:world-context",
            visual_feedback: "a wide unresolved field appears with many dim candidate regions",
        },
        NarrowingStep {
            id: "narrow:market-fit",
            query: "Which market signals are launch-relevant?",
            lattice_operation: "meet active intent with market evidence",
            before_count: 1024,
            after_count: 256,
            saved_chunk: "chunk:launch-readiness-pack",
            frontier: "frontier:excluded-market-noise",
            visual_feedback: "the market region brightens while unrelated signals gray out",
        },
        NarrowingStep {
            id: "narrow:customer-proof",
            query: "Which customer proof belongs in the decision?",
            lattice_operation: "join customer receipts into active cut",
            before_count: 256,
            after_count: 96,
            saved_chunk: "chunk:cited-evidence-map",
            frontier: "frontier:weak-customer-evidence",
            visual_feedback: "customer evidence snaps into the launch craft cabin",
        },
        NarrowingStep {
            id: "narrow:risk-constraints",
            query: "Which risks should constrain the launch?",
            lattice_operation: "apply policy and risk constraints",
            before_count: 96,
            after_count: 32,
            saved_chunk: "chunk:policy-constraint-pack",
            frontier: "frontier:waiver-review",
            visual_feedback: "unsafe routes move behind a visible constraint shield",
        },
        NarrowingStep {
            id: "narrow:execution-frontier",
            query: "What remains unresolved before execution?",
            lattice_operation: "preserve unresolved route frontier",
            before_count: 32,
            after_count: 12,
            saved_chunk: "chunk:implementation-route",
            frontier: "frontier:lattice-execution",
            visual_feedback: "the launch craft forms while unresolved route cargo stays attached",
        },
    ];
    NarrowingTraceReport {
        suite: "WITNESS-narrowing-trace-v1",
        scenario: "product-launch-decision",
        semantic_owner: "LATTICE",
        initial_candidate_count: steps
            .first()
            .map(|step| step.before_count)
            .unwrap_or_default(),
        final_candidate_count: steps
            .last()
            .map(|step| step.after_count)
            .unwrap_or_default(),
        saved_chunk_count: steps
            .iter()
            .filter(|step| step.saved_chunk != "none")
            .count(),
        frontier_count: steps.len(),
        steps,
    }
}

pub fn conversation_surface_fixture() -> ConversationSurfaceReport {
    let turns = vec![
        ConversationTurn {
            id: "turn:001",
            speaker: "user",
            message_summary: "asks whether the product launch is ready",
            harness_event: "intent-captured",
            visual_state: "wide unresolved field opens beside the chat",
            lattice_anchor: "frontier:world-context",
            frontier: "frontier:world-context",
        },
        ConversationTurn {
            id: "turn:002",
            speaker: "assistant",
            message_summary: "asks for launch criteria and relevant source context",
            harness_event: "context-requested",
            visual_state:
                "focus cone highlights market, customer, risk, timing, and readiness regions",
            lattice_anchor: "cut:WITNESS:claude-session:active",
            frontier: "frontier:missing-launch-criteria",
        },
        ConversationTurn {
            id: "turn:003",
            speaker: "user",
            message_summary: "points to market, customer, and risk signals",
            harness_event: "source-pointers-added",
            visual_state: "evidence pieces snap into the launch craft outline",
            lattice_anchor: "receipt:maxim-product-launch-readiness",
            frontier: "frontier:weak-customer-evidence",
        },
        ConversationTurn {
            id: "turn:004",
            speaker: "assistant",
            message_summary: "narrows the decision and names unresolved execution route work",
            harness_event: "response-deltas-folded",
            visual_state: "saved chunks crystallize while unresolved route cargo remains attached",
            lattice_anchor: "chunk:launch-readiness-pack",
            frontier: "frontier:lattice-execution",
        },
        ConversationTurn {
            id: "turn:005",
            speaker: "assistant",
            message_summary: "presents a handoff-ready launch recommendation with visible caveats",
            harness_event: "goal-artifact-projected",
            visual_state: "launch craft forms next to the chat with frontier still visible",
            lattice_anchor: "shape:launch-craft",
            frontier: "frontier:lattice-execution",
        },
    ];
    ConversationSurfaceReport {
        suite: "WITNESS-conversation-surface-v1",
        scenario: "product-launch-decision",
        primary_surface: "chat",
        companion_surface: "context-visualization",
        semantic_owner: "LATTICE",
        turn_count: turns.len(),
        visual_update_count: turns.len(),
        frontier_count: turns
            .iter()
            .filter(|turn| !turn.frontier.is_empty())
            .count(),
        turns,
    }
}

pub fn operator_showcase_fixture() -> OperatorShowcaseReport {
    OperatorShowcaseReport {
        suite: "WITNESS-lattice-operator-showcases-v1",
        semantic_owner: "LATTICE",
        boundary: "LATTICE is extensible at the edges, fixed in the middle; WITNESS shows fixed operators as edge dataflow affordances.",
        generic_dialect_system: false,
        showcases: vec![
            OperatorShowcase {
                id: "search",
                label: "Search",
                scenario: "research",
                input_layer: "SourcePointer",
                operator_role: "find candidate source-backed context for a question",
                output_layer: "CandidateView",
                dataflow: "Source pointers -> search -> candidate evidence field",
                edge_note: "WITNESS displays the query motion; LATTICE owns search semantics.",
            },
            OperatorShowcase {
                id: "close",
                label: "Close",
                scenario: "launch",
                input_layer: "CandidateCut",
                operator_role: "verify a candidate cut under policy and receipts",
                output_layer: "ClosedCut",
                dataflow: "Candidate cut -> close -> handoff-ready closed cut",
                edge_note: "The viewer shows readiness only after LATTICE closure.",
            },
            OperatorShowcase {
                id: "meet",
                label: "Meet",
                scenario: "policy",
                input_layer: "CandidateView",
                operator_role: "intersect compatible constraints or evidence",
                output_layer: "CandidateCut",
                dataflow: "Policy sources + exception rules -> meet -> constrained cut",
                edge_note: "WITNESS renders the meet gate without redefining meet.",
            },
            OperatorShowcase {
                id: "join",
                label: "Join",
                scenario: "route",
                input_layer: "CandidateCut",
                operator_role: "combine compatible evidence paths",
                output_layer: "CandidateCut",
                dataflow: "Dependency cut + validation cut -> join -> route cut",
                edge_note: "WITNESS renders the join spine as display-only dataflow.",
            },
            OperatorShowcase {
                id: "explain",
                label: "Explain",
                scenario: "claim",
                input_layer: "ClosedCut",
                operator_role: "surface why a cut supports a result",
                output_layer: "PromptFrame",
                dataflow: "Closed adjudication cut -> explain -> user-facing rationale",
                edge_note: "Narration is an edge view over LATTICE-owned evidence.",
            },
            OperatorShowcase {
                id: "hasse",
                label: "Hasse",
                scenario: "corpus",
                input_layer: "CandidateView",
                operator_role: "show partial-order relationships without changing them",
                output_layer: "CandidateView",
                dataflow: "Grain relationships -> hasse -> compact ordering view",
                edge_note: "The graph is a visualization, not a new relation language.",
            },
            OperatorShowcase {
                id: "pack",
                label: "Pack",
                scenario: "corpus",
                input_layer: "ClosedCut",
                operator_role: "bundle closed evidence for reuse",
                output_layer: "Pack",
                dataflow: "Closed source cargo -> pack -> reusable context bundle",
                edge_note: "WITNESS shows saved chunks while LATTICE owns pack contracts.",
            },
            OperatorShowcase {
                id: "prompt",
                label: "Prompt",
                scenario: "launch",
                input_layer: "Pack",
                operator_role: "frame a pack for harness consumption",
                output_layer: "PromptFrame",
                dataflow: "Receipt-backed pack -> prompt -> chat-ready frame",
                edge_note: "Provider-facing prompt shape stays at WITNESS's edge.",
            },
            OperatorShowcase {
                id: "validate",
                label: "Validate",
                scenario: "debug",
                input_layer: "Workspace",
                operator_role: "check contracts and execution readiness",
                output_layer: "Receipt",
                dataflow: "Workspace artifacts -> validate -> validation receipt",
                edge_note: "WITNESS shows owed checks; LATTICE owns pass/fail rules.",
            },
            OperatorShowcase {
                id: "rank",
                label: "Rank",
                scenario: "research",
                input_layer: "CandidateView",
                operator_role: "order candidate grains for a selected goal",
                output_layer: "CandidateView",
                dataflow: "Candidate evidence -> rank -> prioritized reading set",
                edge_note: "Ranking is a fixed operator, not an open scoring dialect.",
            },
            OperatorShowcase {
                id: "path",
                label: "Path",
                scenario: "route",
                input_layer: "ClosedCut",
                operator_role: "order source, context, evidence, policy, and receipts",
                output_layer: "PathReport",
                dataflow: "Closed route cut -> path -> ordered reading path",
                edge_note: "WITNESS shows the reading order exactly as LATTICE reports it.",
            },
            OperatorShowcase {
                id: "bridge",
                label: "Bridge",
                scenario: "collaboration",
                input_layer: "ClosedCut",
                operator_role: "find shared anchors between two closed cuts",
                output_layer: "BridgeReport",
                dataflow: "Left cut + right cut -> bridge -> shared anchor map",
                edge_note: "The bridge helps handoff review without adding cross-dialect joins.",
            },
            OperatorShowcase {
                id: "audit",
                label: "Audit",
                scenario: "trace",
                input_layer: "ClosedCut",
                operator_role: "summarize replay, receipt, rights, and source readiness",
                output_layer: "AuditReport",
                dataflow: "Closed trace cut -> audit -> replay-readiness badge",
                edge_note: "WITNESS displays audit badges; LATTICE owns replay evidence.",
            },
            OperatorShowcase {
                id: "project",
                label: "Project",
                scenario: "policy",
                input_layer: "ClosedCut",
                operator_role: "show fixed grain-family rows",
                output_layer: "ProjectionReport",
                dataflow: "Closed policy cut -> project -> fixed family table",
                edge_note: "This is a fixed projection, not a generic projection language.",
            },
            OperatorShowcase {
                id: "compare",
                label: "Compare",
                scenario: "experiment",
                input_layer: "ClosedCut",
                operator_role: "compare overlap, relation, receipt delta, and source sharing",
                output_layer: "CompareReport",
                dataflow: "Baseline cut + result cut -> compare -> overlap report",
                edge_note: "WITNESS shows the report; LATTICE defines comparison fields.",
            },
            OperatorShowcase {
                id: "narrow",
                label: "Narrow",
                scenario: "debug",
                input_layer: "CandidateView",
                operator_role: "reduce candidates under a user or policy constraint",
                output_layer: "CandidateCut",
                dataflow: "Fault field -> narrow -> focused repair candidates",
                edge_note: "WITNESS makes narrowing visible without owning closure.",
            },
            OperatorShowcase {
                id: "frontier",
                label: "Frontier",
                scenario: "incident",
                input_layer: "ClosedCut",
                operator_role: "record unresolved budget, evidence, or custody gaps",
                output_layer: "Frontier",
                dataflow: "Closed incident cut -> frontier -> residual risk cargo",
                edge_note: "Visible frontier keeps unresolved work out of success-shaped UI.",
            },
            OperatorShowcase {
                id: "coverage",
                label: "Coverage",
                scenario: "launch",
                input_layer: "Workspace",
                operator_role: "measure source and evidence coverage",
                output_layer: "CoverageReport",
                dataflow: "Workspace registry -> coverage -> launch readiness coverage",
                edge_note: "WITNESS displays coverage as a harness gate only.",
            },
            OperatorShowcase {
                id: "gaps",
                label: "Gaps",
                scenario: "claim",
                input_layer: "ClosedCut",
                operator_role: "diagnose missing expected grain families or artifacts",
                output_layer: "GapReport",
                dataflow: "Claim cut -> gaps -> missing evidence checklist",
                edge_note: "Gap cards remain diagnostics, not auto-filled semantic defaults.",
            },
            OperatorShowcase {
                id: "diff",
                label: "Diff",
                scenario: "experiment",
                input_layer: "ClosedCut",
                operator_role: "show changed grains or receipts between versions",
                output_layer: "DiffReport",
                dataflow: "Earlier cut + revised cut -> diff -> changed evidence rows",
                edge_note: "WITNESS shows revision deltas while LATTICE owns identity.",
            },
            OperatorShowcase {
                id: "refresh",
                label: "Refresh",
                scenario: "incident",
                input_layer: "Workspace",
                operator_role: "re-evaluate workspace artifacts after source movement",
                output_layer: "Workspace",
                dataflow: "Updated source pointer set -> refresh -> replayable workspace view",
                edge_note: "Refresh stays a fixed workspace pass, not provider-side async work.",
            },
        ],
    }
}

pub fn lattice_scenario_operator_catalog_fixture() -> LatticeScenarioOperatorCatalogReport {
    LatticeScenarioOperatorCatalogReport {
        suite: "WITNESS-lattice-scenario-operators-v1",
        consumed_contract_schema: "lattice.operator-scenario-catalog.v1",
        semantic_owner: "LATTICE",
        edge_viewer: "WITNESS",
        boundary: "WITNESS consumes fixed scenario operator chains for edge display; LATTICE owns semantics, closure, receipts, budgets, and frontier evidence.",
        generic_dialect_system: false,
        scenarios: vec![
            LatticeScenarioOperatorChain {
                id: "launch",
                label: "Launch readiness",
                operators: &["search", "meet", "join", "frontier"],
                summary: "Find readiness evidence, align launch constraints, form the route cut, and preserve deferred execution cargo.",
            },
            LatticeScenarioOperatorChain {
                id: "research",
                label: "Research synthesis",
                operators: &["search", "rank", "explain", "gaps"],
                summary: "Find literature grains, rank evidence, explain support chains, and keep unsupported claims visible.",
            },
            LatticeScenarioOperatorChain {
                id: "debug",
                label: "Debug repair",
                operators: &["narrow", "path", "validate", "diff"],
                summary: "Narrow the fault field, order the repair route, validate replay, and mark changed grains.",
            },
            LatticeScenarioOperatorChain {
                id: "route",
                label: "Route planning",
                operators: &["path", "meet", "join", "coverage"],
                summary: "Order route steps, align dependency gates, form the itinerary, and expose uncovered paths.",
            },
            LatticeScenarioOperatorChain {
                id: "policy",
                label: "Policy review",
                operators: &["audit", "meet", "project", "frontier"],
                summary: "Audit custody, align policy constraints, project review rows, and preserve waiver frontier.",
            },
            LatticeScenarioOperatorChain {
                id: "corpus",
                label: "Corpus reuse",
                operators: &["search", "pack", "refresh", "audit"],
                summary: "Find reusable packs, bundle custody-backed grains, refresh moved sources, and audit cargo.",
            },
            LatticeScenarioOperatorChain {
                id: "trace",
                label: "Trace handoff",
                operators: &["audit", "path", "bridge", "pack"],
                summary: "Audit fixed-spine records, order stages, bridge handoff anchors, and emit a receipt-backed trace.",
            },
            LatticeScenarioOperatorChain {
                id: "incident",
                label: "Incident review",
                operators: &["narrow", "bridge", "refresh", "frontier"],
                summary: "Filter alert noise, bridge owner evidence, rebuild handoff context, and carry residual risk.",
            },
            LatticeScenarioOperatorChain {
                id: "claim",
                label: "Claim review",
                operators: &["validate", "compare", "gaps", "explain"],
                summary: "Validate eligibility evidence, compare decision cuts, find missing evidence, and explain the outcome.",
            },
            LatticeScenarioOperatorChain {
                id: "experiment",
                label: "Experiment replay",
                operators: &["compare", "diff", "refresh", "pack"],
                summary: "Compare the late metric, show the delta, refresh the workspace, and pack follow-ups.",
            },
            LatticeScenarioOperatorChain {
                id: "reporting",
                label: "Data reporting",
                operators: &["search", "project", "compare", "coverage"],
                summary: "Find reporting grains, project aggregate rows, compare partition/detail cuts, and keep coverage gaps visible.",
            },
        ],
    }
}

pub fn delta_engine_coverage_fixture() -> DeltaCoverageReport {
    let replay = claude_session_fixture();
    let deltas = response_delta_fixture();
    DeltaCoverageReport {
        fixture: "claude-session",
        engine: "WITNESS LATTICE adapter",
        levels: vec![
            DeltaCoverageRow {
                level: DeltaCoverageLevel::L0,
                name: "harness-event capture",
                contract: "raw harness lifecycle events with source pointers, receipts, frontier, checkpoint, and rehydrate",
                evidence_count: replay.event_count(),
                complete: replay.event_count() >= 7 && replay.source_pointer_count() >= 3,
            },
            DeltaCoverageRow {
                level: DeltaCoverageLevel::L1,
                name: "response-delta intent",
                contract: "AI response emits high-level deltas instead of low-level lattice expressions",
                evidence_count: deltas.delta_count(),
                complete: deltas.delta_count() >= 7,
            },
            DeltaCoverageRow {
                level: DeltaCoverageLevel::L2,
                name: "folded lattice-operation events",
                contract: "WITNESS folds deltas into receipt-ready context events including meet/join operation intents",
                evidence_count: deltas.folded_event_count(),
                complete: deltas
                    .deltas
                    .iter()
                    .any(|delta| delta.kind == ResponseDeltaKind::NarrowContext)
                    && deltas
                        .deltas
                        .iter()
                        .any(|delta| delta.kind == ResponseDeltaKind::JoinContext),
            },
        ],
    }
}

pub fn interaction_scenario_suite(provider_adapter: &str) -> InteractionScenarioReport {
    let scenarios = interaction_scenarios();
    InteractionScenarioReport {
        suite: "WITNESS-real-interaction-v1",
        provider_adapter: provider_adapter.to_string(),
        base_llm_decision_maker: provider_adapter.to_string(),
        provider_execution: "base_llm_decision_adapter_contract",
        decision_boundary:
            "base LLM proposes decisions and high-level deltas; WITNESS folds, gates, records, and prepares LATTICE handoff",
        scenario_count: scenarios.len(),
        family_count: INTERACTION_FAMILIES.len(),
        l0_covered_count: scenarios.len(),
        l1_covered_count: scenarios.len(),
        l2_covered_count: scenarios
            .iter()
            .filter(|scenario| {
                matches!(
                    scenario.expected_delta,
                    ResponseDeltaKind::NarrowContext | ResponseDeltaKind::JoinContext
                )
            })
            .count(),
        permission_gate_count: scenarios
            .iter()
            .filter(|scenario| scenario.required_gate != "none")
            .count(),
        source_grounded_count: scenarios
            .iter()
            .filter(|scenario| {
                matches!(
                    scenario.expected_delta,
                    ResponseDeltaKind::AddSource | ResponseDeltaKind::JoinContext
                )
            })
            .count(),
        checkpoint_count: scenarios
            .iter()
            .filter(|scenario| scenario.expected_delta == ResponseDeltaKind::Checkpoint)
            .count(),
        scenarios,
    }
}

fn interaction_scenarios() -> Vec<InteractionScenario> {
    let mut scenarios = Vec::new();
    for (family_index, family) in INTERACTION_FAMILIES.iter().enumerate() {
        for (case_index, case) in INTERACTION_CASES.iter().enumerate() {
            let number = family_index * INTERACTION_CASES.len() + case_index + 1;
            scenarios.push(InteractionScenario {
                id: format!("interaction-{number:03}"),
                family: family.id,
                title: format!("{}: {}", family.title, case.title),
                user_intent: format!("{} {}", family.intent_prefix, case.intent),
                expected_delta: case.delta,
                required_gate: case.gate,
            });
        }
    }
    scenarios
}

struct InteractionFamily {
    id: &'static str,
    title: &'static str,
    intent_prefix: &'static str,
}

struct InteractionCase {
    title: &'static str,
    intent: &'static str,
    delta: ResponseDeltaKind,
    gate: &'static str,
}

const INTERACTION_FAMILIES: [InteractionFamily; 10] = [
    InteractionFamily {
        id: "bootstrap",
        title: "Bootstrap context",
        intent_prefix: "Start a new coding session and",
    },
    InteractionFamily {
        id: "permissions",
        title: "Tool permission gates",
        intent_prefix: "Handle a tool request and",
    },
    InteractionFamily {
        id: "file-navigation",
        title: "File navigation",
        intent_prefix: "Inspect workspace files and",
    },
    InteractionFamily {
        id: "edit-cycle",
        title: "Edit cycle",
        intent_prefix: "Prepare a source change and",
    },
    InteractionFamily {
        id: "validation",
        title: "Validation loop",
        intent_prefix: "Validate the current work and",
    },
    InteractionFamily {
        id: "checkpoint",
        title: "Checkpoint and resume",
        intent_prefix: "Preserve long-running state and",
    },
    InteractionFamily {
        id: "source-corpus",
        title: "Source-corpus grounding",
        intent_prefix: "Use external source context and",
    },
    InteractionFamily {
        id: "repair",
        title: "Conversation repair",
        intent_prefix: "Correct a prior assumption and",
    },
    InteractionFamily {
        id: "collaboration",
        title: "Collaborative handoff",
        intent_prefix: "Coordinate with another reviewer and",
    },
    InteractionFamily {
        id: "lattice-phase",
        title: "LATTICE phase control",
        intent_prefix: "Move through a context phase and",
    },
];

const INTERACTION_CASES: [InteractionCase; 10] = [
    InteractionCase {
        title: "keep active goal",
        intent: "keep the active user goal stable.",
        delta: ResponseDeltaKind::KeepIntent,
        gate: "none",
    },
    InteractionCase {
        title: "add repo instructions",
        intent: "add governing repo instructions as source pointers.",
        delta: ResponseDeltaKind::AddSource,
        gate: "read-allowed",
    },
    InteractionCase {
        title: "narrow to docs",
        intent: "narrow the active work to documentation only.",
        delta: ResponseDeltaKind::NarrowContext,
        gate: "read-allowed",
    },
    InteractionCase {
        title: "join implementation context",
        intent: "join implementation evidence with the active task.",
        delta: ResponseDeltaKind::JoinContext,
        gate: "read-allowed",
    },
    InteractionCase {
        title: "frontier unsafe material",
        intent: "frontier unresolved or unsafe context.",
        delta: ResponseDeltaKind::FrontierGap,
        gate: "privacy-boundary",
    },
    InteractionCase {
        title: "record validation obligation",
        intent: "record validation owed before completion.",
        delta: ResponseDeltaKind::ValidationOwed,
        gate: "command-allowed",
    },
    InteractionCase {
        title: "checkpoint progress",
        intent: "checkpoint the folded session state.",
        delta: ResponseDeltaKind::Checkpoint,
        gate: "write-allowed",
    },
    InteractionCase {
        title: "add source-corpus evidence",
        intent: "add source-corpus evidence without copying opaque blobs.",
        delta: ResponseDeltaKind::AddSource,
        gate: "source-custody",
    },
    InteractionCase {
        title: "narrow after user correction",
        intent: "narrow around a corrected user premise.",
        delta: ResponseDeltaKind::NarrowContext,
        gate: "none",
    },
    InteractionCase {
        title: "join handoff summary",
        intent: "join the current cut with a handoff summary.",
        delta: ResponseDeltaKind::JoinContext,
        gate: "write-allowed",
    },
];

fn optional_json_string(value: Option<&str>) -> String {
    match value {
        Some(value) => format!("\"{}\"", escape_json(value)),
        None => "null".to_string(),
    }
}

fn json_string_array(values: &[&str]) -> String {
    values
        .iter()
        .map(|value| format!("\"{}\"", escape_json(value)))
        .collect::<Vec<_>>()
        .join(",")
}

fn escape_json(value: &str) -> String {
    value
        .replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
}

fn stable_checksum(value: &str) -> u64 {
    value.bytes().fold(0xcbf29ce484222325, |hash, byte| {
        (hash ^ u64::from(byte)).wrapping_mul(0x100000001b3)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn claude_session_fixture_tracks_harness_lifecycle() {
        let replay = claude_session_fixture();

        assert_eq!(replay.event_count(), 7);
        assert_eq!(replay.source_pointer_count(), 3);
        assert_eq!(replay.receipt_count(), 7);
        assert_eq!(replay.frontier_count, 2);
    }

    #[test]
    fn claude_session_fixture_json_reports_rehydrate_contract() {
        let output = claude_session_fixture().to_json();

        assert!(output.contains("\"schema\":\"witness.harness.v1\""));
        assert!(output.contains("\"fixture\":\"claude-session\""));
        assert!(output.contains("\"kind\":\"bootstrap_context\""));
        assert!(output.contains("\"kind\":\"rehydrate\""));
        assert!(output.contains("\"substrate\":\"LATTICE closed-cut contract\""));
    }

    #[test]
    fn response_delta_fixture_folds_high_level_ai_deltas() {
        let plan = response_delta_fixture();

        assert_eq!(plan.delta_count(), 7);
        assert_eq!(plan.folded_event_count(), 7);
        assert!(plan
            .deltas
            .iter()
            .any(|delta| delta.kind == ResponseDeltaKind::NarrowContext));
        assert!(plan
            .deltas
            .iter()
            .any(|delta| delta.kind == ResponseDeltaKind::JoinContext));
    }

    #[test]
    fn response_delta_json_reports_folded_lattice_operations() {
        let output = response_delta_fixture().to_json();

        assert!(output.contains("\"schema\":\"witness.response-deltas.v1\""));
        assert!(output.contains("\"language\":\"LATTICE\""));
        assert!(output.contains("\"adapter\":\"WITNESS\""));
        assert!(output.contains("\"kind\":\"narrow_context\""));
        assert!(output.contains("\"folded_operation\":\"meet-with-constraint\""));
        assert!(output.contains("\"folded_operation\":\"join-with-source-cut\""));
        assert!(output.contains("\"kind\":\"lattice_operation\""));
    }

    #[test]
    fn lattice_delta_projection_json_names_lattice_as_language_owner() {
        let output = response_delta_fixture().to_lattice_delta_projection_json();

        assert!(output.contains("\"schema\":\"witness.lattice-delta-projection.v1\""));
        assert!(output.contains("\"language\":\"LATTICE\""));
        assert!(output.contains("\"language_owner\":\"LATTICE\""));
        assert!(output.contains("\"adapter\":\"WITNESS\""));
        assert!(output.contains("\"delta_count\":7"));
    }

    #[test]
    fn delta_engine_coverage_reports_l0_l1_l2() {
        let coverage = delta_engine_coverage_fixture();

        assert_eq!(coverage.level_count(), 3);
        assert_eq!(coverage.complete_count(), 3);
        assert_eq!(coverage.total_evidence_count(), 21);
    }

    #[test]
    fn delta_engine_coverage_json_names_contract_layers() {
        let output = delta_engine_coverage_fixture().to_json();

        assert!(output.contains("\"schema\":\"witness.delta-coverage.v1\""));
        assert!(output.contains("\"level\":\"L0\""));
        assert!(output.contains("\"level\":\"L1\""));
        assert!(output.contains("\"level\":\"L2\""));
        assert!(output.contains("\"engine\":\"WITNESS LATTICE adapter\""));
        assert!(output.contains("\"complete_count\":3"));
    }

    #[test]
    fn interaction_suite_reports_100_real_interaction_scenarios() {
        let report = interaction_scenario_suite("claude-p");

        assert_eq!(report.scenario_count, 100);
        assert_eq!(report.family_count, 10);
        assert_eq!(report.l0_covered_count, 100);
        assert_eq!(report.l1_covered_count, 100);
        assert_eq!(report.l2_covered_count, 40);
        assert_eq!(report.source_grounded_count, 40);
        assert_eq!(report.checkpoint_count, 10);
    }

    #[test]
    fn interaction_suite_json_names_claude_p_adapter_without_invocation() {
        let output = interaction_scenario_suite("claude-p").to_json();

        assert!(output.contains("\"schema\":\"witness.interaction-scenarios.v1\""));
        assert!(output.contains("\"provider_adapter\":\"claude-p\""));
        assert!(output.contains("\"base_llm_decision_maker\":\"claude-p\""));
        assert!(output.contains("\"provider_execution\":\"base_llm_decision_adapter_contract\""));
        assert!(output.contains("base LLM proposes decisions and high-level deltas"));
        assert!(output.contains("\"scenario_count\":100"));
        assert!(output.contains("\"id\":\"interaction-100\""));
    }

    #[test]
    fn lattice_handoff_fixture_names_lattice_owned_contracts() {
        let handoff = lattice_handoff_fixture();

        assert_eq!(handoff.language_owner, "LATTICE");
        assert_eq!(handoff.source_pointer_count(), 3);
        assert_eq!(handoff.cut_count(), 3);
        assert_eq!(handoff.receipt_count(), 7);
        assert_eq!(handoff.frontier_count(), 2);
        assert_eq!(handoff.operation_intent_count(), 2);
        assert_eq!(
            handoff.lattice_replay_command,
            "lattice-cli handoff replay tiny --json"
        );
        assert_eq!(handoff.lattice_replay_schema, "lattice.handoff-replay.v1");
        assert_eq!(handoff.lattice_frontier_status, "frontier_deferred");
    }

    #[test]
    fn lattice_handoff_json_keeps_closure_with_lattice() {
        let output = lattice_handoff_fixture().to_json();

        assert!(output.contains("\"schema\":\"witness.lattice-handoff.v1\""));
        assert!(output.contains("\"language_owner\":\"LATTICE\""));
        assert!(output.contains("LATTICE owns closure"));
        assert!(output.contains("\"custody\":\"pointer-only\""));
        assert!(output.contains("\"operation\":\"meet\""));
        assert!(output.contains("\"operation\":\"join\""));
        assert!(output.contains("\"status\":\"handoff-request\""));
        assert!(output.contains("\"lattice_replay_schema\":\"lattice.handoff-replay.v1\""));
        assert!(output.contains("\"lattice_delta_schema\":\"lattice.delta.v1\""));
        assert!(output.contains("\"lattice_frontier_status\":\"frontier_deferred\""));
        assert!(output.contains("LATTICE receipt lattice-stable-v1:e44e5cc6a36f0cf3"));
    }

    #[test]
    fn lattice_source_pilot_json_displays_pointer_only_metadata() {
        let output = lattice_source_pilot_fixture().to_json();

        assert!(output.contains("\"schema\":\"witness.lattice-source-pilot.v1\""));
        assert!(output.contains("lattice-cli registry pilot fontes"));
        assert!(output.contains("\"semantic_owner\":\"LATTICE\""));
        assert!(output.contains("\"source_id\":\"fontes:apache-calcite:query-planning\""));
        assert!(output.contains("\"owner_repo\":\"giodl73-repo/FONTES\""));
        assert!(output.contains("\"metadata_only\":true"));
        assert!(output.contains("\"source_content_copied\":false"));
        assert!(output.contains("registry-add-fontes-apache-calcite-query-planning"));
    }

    #[test]
    fn mechanical_sandbox_json_previews_exact_lattice_delta() {
        let output = mechanical_sandbox_fixture().to_json();

        assert!(output.contains("\"schema\":\"witness.mechanical-sandbox.v1\""));
        assert!(output.contains("\"provider_execution\":\"none-fixture-backed\""));
        assert!(output.contains("\"consumed_lattice_schema\":\"lattice.delta.v1\""));
        assert!(output.contains("meet(active_literal_cut, condition:quiet-parks)"));
        assert!(output.contains("\"folded_operation\":\"meet-with-constraint\""));
        assert!(output.contains("reviewed-turn-only"));
        assert!(output.contains("\"handoff_ready\":true"));
    }

    #[test]
    fn reviewer_workflow_json_defaults_to_receipt_backed_review_path() {
        let output = reviewer_workflow_fixture().to_json();

        assert!(output.contains("\"schema\":\"witness.reviewer-workflow.v1\""));
        assert!(output.contains("\"default_route\":\"handoff-review\""));
        assert!(output.contains("\"default_graph_view\":\"evidence\""));
        assert!(output.contains("\"boundary-view\""));
        assert!(output.contains("\"evidence-trail\""));
        assert!(output.contains("\"receipt-citation\""));
        assert!(output.contains("\"receipt_required\":true"));
        assert!(output.contains("\"provider_execution\":\"none-review-only\""));
    }

    #[test]
    fn press_preview_json_keeps_rendering_at_press_edge() {
        let output = press_preview_fixture().to_json();

        assert!(output.contains("\"schema\":\"witness.press-preview.v1\""));
        assert!(output.contains("\"consumed_contract_schema\":\"lattice.press-frame.v1\""));
        assert!(output.contains("\"semantic_owner\":\"LATTICE\""));
        assert!(output.contains("\"preview_owner\":\"WITNESS\""));
        assert!(output.contains("\"renderer_owner\":\"PRESS\""));
        assert!(output.contains("\"witness_renders_publication\":false"));
        assert!(output.contains("\"lattice_renders_publication\":false"));
        assert!(output.contains("\"md\""));
        assert!(output.contains("\"html\""));
    }

    #[test]
    fn lattice_algebra_validation_json_displays_lattice_owned_property_proof() {
        let report = lattice_algebra_validation_fixture();
        let output = report.to_json();

        assert_eq!(report.property_count(), 46);
        assert!(report.passed);
        assert!(output.contains("\"schema\":\"witness.lattice-algebra-validation.v1\""));
        assert!(output.contains("lattice-cli validate algebra --json"));
        assert!(output
            .contains("\"consumed_contract_schema\":\"lattice.algebra-property-validation.v1\""));
        assert!(output.contains("\"semantic_owner\":\"LATTICE\""));
        assert!(output.contains("\"viewer_role\":\"property-proof display\""));
        assert!(output.contains("\"id\":\"meet_commutative\""));
        assert!(output.contains("\"id\":\"join_absorption\""));
        assert!(output.contains("\"id\":\"meet_associative\""));
        assert!(output.contains("\"id\":\"join_associative\""));
        assert!(output.contains("\"id\":\"meet_lower_bound_left\""));
        assert!(output.contains("\"id\":\"join_upper_bound_right\""));
        assert!(output.contains("\"id\":\"meet_monotone_left\""));
        assert!(output.contains("\"id\":\"join_monotone_right\""));
        assert!(output.contains("\"id\":\"meet_closure_witness\""));
        assert!(output.contains("\"id\":\"join_closure_witness\""));
        assert!(output.contains("\"id\":\"meet_policy_witness\""));
        assert!(output.contains("\"id\":\"join_policy_witness\""));
        assert!(output.contains("\"id\":\"meet_rights_witness\""));
        assert!(output.contains("\"id\":\"join_rights_witness\""));
        assert!(output.contains("\"id\":\"meet_bond_endpoint_witness\""));
        assert!(output.contains("\"id\":\"join_bond_endpoint_witness\""));
        assert!(output.contains("\"id\":\"meet_deterministic_replay\""));
        assert!(output.contains("\"id\":\"join_deterministic_replay\""));
        assert!(output.contains("\"id\":\"meet_input_closure_witness\""));
        assert!(output.contains("\"id\":\"join_input_closure_witness\""));
        assert!(output.contains("\"id\":\"meet_grain_set_witness\""));
        assert!(output.contains("\"id\":\"join_grain_set_witness\""));
        assert!(output.contains("\"id\":\"meet_bond_set_witness\""));
        assert!(output.contains("\"id\":\"join_bond_set_witness\""));
        assert!(output.contains("\"id\":\"meet_receipt_note_witness\""));
        assert!(output.contains("\"id\":\"join_receipt_note_witness\""));
        assert!(output.contains("\"id\":\"meet_receipt_hash_witness\""));
        assert!(output.contains("\"id\":\"join_receipt_hash_witness\""));
        assert!(output.contains("\"id\":\"meet_result_id_witness\""));
        assert!(output.contains("\"id\":\"join_result_id_witness\""));
        assert!(output.contains("\"id\":\"meet_receipt_count_witness\""));
        assert!(output.contains("\"id\":\"join_receipt_count_witness\""));
        assert!(output.contains("\"id\":\"meet_policy_value_witness\""));
        assert!(output.contains("\"id\":\"join_policy_value_witness\""));
        assert!(output.contains("\"id\":\"meet_rights_value_witness\""));
        assert!(output.contains("\"id\":\"join_rights_value_witness\""));
        assert!(output.contains("\"id\":\"meet_receipt_rule_witness\""));
        assert!(output.contains("\"id\":\"join_receipt_rule_witness\""));
        assert!(output.contains("\"property_count\":46"));
        assert!(output.contains("\"passed\":true"));
    }

    #[test]
    fn lattice_shard_display_json_consumes_lattice_owned_alignment_report() {
        let report = lattice_shard_display_fixture();
        let output = report.to_json();

        assert_eq!(report.semantic_owner, "LATTICE");
        assert_eq!(report.shard_count, 5);
        assert_eq!(report.boundary_edge_count, 6);
        assert_eq!(report.alignment_row_count, 5);
        assert_eq!(report.conflicting_boundary_count, 1);
        assert!(!report.witness_recomputes_alignment);
        assert!(output.contains("\"schema\":\"witness.lattice-shard-display.v1\""));
        assert!(output.contains("lattice-cli materialize shards demo --json"));
        assert!(output.contains("\"consumed_contract_schema\":\"lattice.shard-alignment-demo.v1\""));
        assert!(output.contains("\"aggregate_hash\":\"lattice-stable-v1:d2b94725746a2216\""));
        assert!(output.contains("\"witness_recomputes_alignment\":false"));
        assert!(output.contains("\"shard_id\":\"engineering-notes-2026-q2\""));
        assert!(output.contains("\"alignment_status\":\"conflict\""));
        assert!(output.contains("\"row_id\":\"conflicts_not_flattened\""));
    }

    #[test]
    fn lattice_shard_scenarios_json_displays_lattice_owned_examples() {
        let report = lattice_shard_scenarios_fixture();
        let output = report.to_json();

        assert_eq!(report.semantic_owner, "LATTICE");
        assert_eq!(report.scenario_count, 5);
        assert!(!report.witness_recomputes_routing);
        assert!(output.contains("\"schema\":\"witness.lattice-shard-scenarios.v1\""));
        assert!(output.contains("lattice-cli shards scenarios --json"));
        assert!(output.contains("\"consumed_contract_schema\":\"lattice.shard-scenario-set.v1\""));
        assert!(output.contains("\"scenario_hash\":\"lattice-stable-v1:ab04b3ad46ccdc5f\""));
        assert!(output.contains("\"witness_recomputes_routing\":false"));
        assert!(output.contains("\"scenario_id\":\"support-conflict-route\""));
        assert!(output.contains("\"scenario_id\":\"coverage-gap-sweep\""));
        assert!(output.contains("edge-support-engineering-conflict"));
    }

    #[test]
    fn lattice_shard_validation_json_displays_lattice_owned_artifact_checks() {
        let report = lattice_shard_validation_fixture();
        let output = report.to_json();

        assert_eq!(report.semantic_owner, "LATTICE");
        assert_eq!(report.check_count, 6);
        assert!(report.validation_passed);
        assert!(!report.witness_recomputes_validation);
        assert!(output.contains("\"schema\":\"witness.lattice-shard-validation.v1\""));
        assert!(output.contains("lattice-cli validate shards --workspace .lattice-shards --json"));
        assert!(output
            .contains("\"consumed_contract_schema\":\"lattice.shard-artifact-validation.v1\""));
        assert!(output.contains("\"witness_recomputes_validation\":false"));
        assert!(output.contains("\"name\":\"aggregate-hash-replay\""));
        assert!(output.contains("\"name\":\"summary-counts-replay\""));
        assert!(output.contains("\"name\":\"fixed-middle-boundary\""));
        assert!(output.contains("global_closure_claimed:false"));
    }

    #[test]
    fn format_matrix_models_customer_harness_needs() {
        let report = format_matrix_fixture();

        assert_eq!(report.target_count(), 5);
        assert_eq!(report.average_projection_fidelity(), 86);
        assert!(report.rows.iter().all(|row| row.projection_fidelity >= 80));
    }

    #[test]
    fn format_matrix_json_names_lattice_validation_surface() {
        let output = format_matrix_fixture().to_json();

        assert!(output.contains("\"schema\":\"witness.format-matrix.v1\""));
        assert!(output.contains("\"target\":\"claude-code\""));
        assert!(output.contains("\"target\":\"copilot-agent\""));
        assert!(output.contains("validate LATTICE contracts"));
        assert!(output.contains("\"lattice_validation\""));
        assert!(output.contains("\"target_count\":5"));
    }

    #[test]
    fn checkpoint_artifact_preserves_rehydration_inputs() {
        let artifact = checkpoint_artifact_fixture();

        assert_eq!(artifact.fixture, "claude-session");
        assert_eq!(artifact.receipt_count, 7);
        assert_eq!(artifact.frontier_count, 2);
        assert_eq!(artifact.source_pointer_count, 3);
        assert_eq!(artifact.custody, "local-artifact-pointer-only");
    }

    #[test]
    fn checkpoint_write_report_matches_artifact_content() {
        let report = checkpoint_write_report("target/witness-checkpoints/claude-session.json");
        let artifact = report.artifact_json();

        assert_eq!(report.byte_count, artifact.len());
        assert_eq!(report.checksum, stable_checksum(&artifact));
        assert_eq!(report.rehydration_status, "ready-from-receipts");
        assert!(report
            .to_json()
            .contains("\"schema\":\"witness.checkpoint-write.v1\""));
        assert!(artifact.contains("\"schema\":\"witness.checkpoint-artifact.v1\""));
    }

    #[test]
    fn session_corpus_review_requires_all_safety_gates() {
        let report = session_corpus_review_fixture();

        assert!(!report.raw_ingestion_allowed);
        assert_eq!(report.gate_count, 6);
        assert_eq!(report.approved_gate_count, 6);
        assert!(report.approved());
    }

    #[test]
    fn session_corpus_review_json_keeps_raw_logs_out_of_truth() {
        let output = session_corpus_review_fixture().to_json();

        assert!(output.contains("\"schema\":\"witness.session-corpus-review.v1\""));
        assert!(output.contains("\"raw_ingestion_allowed\":false"));
        assert!(output.contains("\"id\":\"secret-exclusion\""));
        assert!(output.contains("\"id\":\"raw-chat-minimization\""));
        assert!(output.contains("\"approved\":true"));
        assert!(output.contains("source pointer and custody receipt"));
    }

    #[test]
    fn maxim_scenarios_use_pointers_and_receipts() {
        let report = maxim_scenario_fixture();

        assert_eq!(report.corpus, "MAXIM");
        assert_eq!(report.custody, "pointer-only");
        assert_eq!(report.scenario_count, 6);
        assert_eq!(report.pointer_count, 6);
        assert_eq!(report.receipt_count, 6);
        assert_eq!(report.frontier_count, 2);
    }

    #[test]
    fn maxim_scenarios_json_names_goal_shapes() {
        let output = maxim_scenario_fixture().to_json();

        assert!(output.contains("\"schema\":\"witness.maxim-scenarios.v1\""));
        assert!(output.contains("\"result_shape\":\"launch-craft\""));
        assert!(output.contains("\"result_shape\":\"cargo-crates\""));
        assert!(output.contains("\"saved_chunk\":\"chunk:launch-readiness-pack\""));
        assert!(output.contains("\"custody\":\"pointer-only\""));
    }

    #[test]
    fn maxim_conversations_exercise_chat_visual_and_source_gates() {
        let report = maxim_conversation_fixture();

        assert_eq!(report.corpus, "MAXIM");
        assert_eq!(report.provider_mode, "deterministic-no-provider-call");
        assert_eq!(report.custody, "pointer-only");
        assert_eq!(report.semantic_owner, "LATTICE");
        assert_eq!(report.conversation_count, 3);
        assert_eq!(report.turn_count, 12);
        assert_eq!(report.pointer_count, 3);
        assert_eq!(report.receipt_count, 3);
        assert_eq!(report.visual_update_count, 12);
        assert_eq!(report.passed_gate_count, 3);
    }

    #[test]
    fn maxim_conversations_json_names_real_harness_conversation_shapes() {
        let output = maxim_conversation_fixture().to_json();

        assert!(output.contains("\"schema\":\"witness.maxim-conversations.v1\""));
        assert!(output.contains("\"title\":\"Launch readiness conversation\""));
        assert!(output.contains("\"expected_shape\":\"evidence-constellation\""));
        assert!(output.contains("\"harness_event\":\"frontier-preserved\""));
        assert!(output.contains("\"provider_mode\":\"deterministic-no-provider-call\""));
        assert!(output.contains("\"semantic_owner\":\"LATTICE\""));
    }

    #[test]
    fn maxim_conversation_eval_scores_all_gates() {
        let eval = maxim_conversation_eval_fixture();

        assert_eq!(eval.corpus, "MAXIM");
        assert_eq!(eval.evaluation_target, "witness.maxim-conversations.v1");
        assert_eq!(eval.semantic_owner, "LATTICE");
        assert_eq!(eval.scenario_count, 3);
        assert_eq!(eval.passed_count, 3);
        assert_eq!(eval.failed_count, 0);
        assert_eq!(eval.gate_count, 15);
    }

    #[test]
    fn maxim_conversation_eval_json_names_gate_results() {
        let output = maxim_conversation_eval_fixture().to_json();

        assert!(output.contains("\"schema\":\"witness.maxim-conversation-eval.v1\""));
        assert!(output.contains("\"pointer_gate\":\"passed:maxim-pointer-present\""));
        assert!(output.contains("\"visual_gate\":\"passed:visual-update-per-turn\""));
        assert!(output.contains("\"shape_gate\":\"passed:route-lattice\""));
        assert!(output.contains("\"failed_count\":0"));
        assert!(output.contains("\"semantic_owner\":\"LATTICE\""));
    }

    #[test]
    fn maxim_conversation_replay_tracks_turn_visual_frames() {
        let replay = maxim_conversation_replay_fixture();

        assert_eq!(replay.corpus, "MAXIM");
        assert_eq!(replay.semantic_owner, "LATTICE");
        assert_eq!(replay.frame_count, 6);
        assert_eq!(replay.visual_frame_count, 6);
        assert_eq!(replay.source_anchor_count, 1);
        assert_eq!(replay.frontier_frame_count, 2);
    }

    #[test]
    fn maxim_conversation_replay_json_names_transcript_and_assertions() {
        let output = maxim_conversation_replay_fixture().to_json();

        assert!(output.contains("\"schema\":\"witness.maxim-conversation-replay.v1\""));
        assert!(output.contains("\"transcript\":\"maxim-launch-readiness-chat\""));
        assert!(output.contains("\"expected_delta\":\"add_source\""));
        assert!(output.contains("\"visual_frame\":\"launch-craft-formed-with-frontier-cargo\""));
        assert!(output.contains("MAXIM enters as a pointer"));
        assert!(output.contains("\"semantic_owner\":\"LATTICE\""));
    }

    #[test]
    fn maxim_conversation_sync_checks_chat_visual_lockstep() {
        let sync = maxim_conversation_sync_fixture();

        assert_eq!(sync.transcript, "maxim-launch-readiness-chat");
        assert_eq!(sync.sync_target, "chat-surface-plus-context-visualization");
        assert_eq!(sync.semantic_owner, "LATTICE");
        assert_eq!(sync.check_count, 6);
        assert_eq!(sync.passed_count, 6);
        assert_eq!(sync.failed_count, 0);
    }

    #[test]
    fn maxim_conversation_sync_json_names_visual_contracts() {
        let output = maxim_conversation_sync_fixture().to_json();

        assert!(output.contains("\"schema\":\"witness.maxim-conversation-sync.v1\""));
        assert!(output.contains("\"visual_contract\":\"witness.conversation-surface.v1\""));
        assert!(output.contains("\"visual_contract\":\"witness.artifact-assembly.v1\""));
        assert!(output.contains("\"sync_gate\":\"passed:artifact-with-frontier\""));
        assert!(output.contains("\"failed_count\":0"));
        assert!(output.contains("\"semantic_owner\":\"LATTICE\""));
    }

    #[test]
    fn chat_web_readiness_names_routes_and_privacy_gates() {
        let readiness = chat_web_readiness_fixture();

        assert_eq!(
            readiness.intended_command,
            "witness-cli chat web --port 8787"
        );
        assert_eq!(readiness.host_policy, "localhost-only-fixture-backed");
        assert_eq!(readiness.semantic_owner, "LATTICE");
        assert_eq!(readiness.route_count, 6);
        assert_eq!(readiness.data_feed_count, 6);
        assert_eq!(readiness.privacy_gate_count, 6);
        assert_eq!(readiness.ready_gate_count, 6);
    }

    #[test]
    fn chat_web_readiness_json_lists_web_feeds() {
        let output = chat_web_readiness_fixture().to_json();

        assert!(output.contains("\"schema\":\"witness.chat-web-readiness.v1\""));
        assert!(output.contains("\"path\":\"/api/chat/replay\""));
        assert!(output.contains("\"data_feed\":\"witness.maxim-conversation-sync.v1\""));
        assert!(output.contains("\"render_state\":\"launch craft with frontier cargo\""));
        assert!(output.contains("\"privacy_gate\":\"no-raw-source\""));
        assert!(output.contains("\"semantic_owner\":\"LATTICE\""));
    }

    #[test]
    fn chat_web_fixture_bundle_collects_bootstrap_feeds() {
        let bundle = chat_web_fixture_bundle();

        assert_eq!(bundle.bootstrap_route, "/api/bootstrap");
        assert_eq!(bundle.host_policy, "localhost-only-fixture-backed");
        assert_eq!(bundle.semantic_owner, "LATTICE");
        assert_eq!(bundle.feed_count, 7);
        assert_eq!(
            bundle.privacy_gate,
            "pointer-only-no-provider-no-raw-source"
        );
    }

    #[test]
    fn chat_web_fixture_bundle_json_embeds_render_payloads() {
        let output = chat_web_fixture_bundle().to_json();

        assert!(output.contains("\"schema\":\"witness.chat-web-fixture.v1\""));
        assert!(output.contains("\"bootstrap_route\":\"/api/bootstrap\""));
        assert!(output.contains("\"web_ready\":{\"schema\":\"witness.chat-web-readiness.v1\""));
        assert!(output
            .contains("\"maxim_replay\":{\"schema\":\"witness.maxim-conversation-replay.v1\""));
        assert!(
            output.contains("\"artifact_assembly\":{\"schema\":\"witness.artifact-assembly.v1\"")
        );
        assert!(output.contains("\"privacy_gate\":\"pointer-only-no-provider-no-raw-source\""));
    }

    #[test]
    fn chat_web_storyboard_names_panels_and_frames() {
        let storyboard = chat_web_storyboard_fixture();

        assert_eq!(storyboard.target_surface, "chat-web-localhost");
        assert_eq!(storyboard.semantic_owner, "LATTICE");
        assert_eq!(storyboard.panel_count, 5);
        assert_eq!(storyboard.frame_count, 6);
        assert!(storyboard
            .panels
            .iter()
            .any(|panel| panel.id == "panel:artifact"));
    }

    #[test]
    fn chat_web_storyboard_json_maps_replay_to_visual_cues() {
        let output = chat_web_storyboard_fixture().to_json();

        assert!(output.contains("\"schema\":\"witness.chat-web-storyboard.v1\""));
        assert!(output.contains("\"active_panel\":\"panel:context-field\""));
        assert!(output.contains(
            "\"animation_cue\":\"launch craft forms with frontier cargo still connected\""
        ));
        assert!(output.contains("\"frontier_state\":\"frontier:lattice-execution\""));
        assert!(output.contains("\"semantic_owner\":\"LATTICE\""));
    }

    #[test]
    fn chat_web_acceptance_names_required_server_gates() {
        let acceptance = chat_web_acceptance_fixture();

        assert_eq!(
            acceptance.target_command,
            "witness-cli chat web --port 8787"
        );
        assert_eq!(acceptance.semantic_owner, "LATTICE");
        assert_eq!(acceptance.criterion_count, 6);
        assert_eq!(acceptance.required_count, 6);
        assert!(acceptance
            .criteria
            .iter()
            .any(|criterion| criterion.id == "accept:safety"));
    }

    #[test]
    fn chat_web_acceptance_json_lists_bootstrap_and_safety_checks() {
        let output = chat_web_acceptance_fixture().to_json();

        assert!(output.contains("\"schema\":\"witness.chat-web-acceptance.v1\""));
        assert!(output.contains("\"target_command\":\"witness-cli chat web --port 8787\""));
        assert!(output.contains("\"evidence_feed\":\"witness.chat-web-fixture.v1\""));
        assert!(output.contains(
            "\"expected_check\":\"privacy_gate is pointer-only-no-provider-no-raw-source\""
        ));
        assert!(output.contains("\"required_count\":6"));
        assert!(output.contains("\"semantic_owner\":\"LATTICE\""));
    }

    #[test]
    fn visual_context_fixture_projects_lattice_motion() {
        let report = visual_context_fixture();

        assert_eq!(report.semantic_owner, "LATTICE");
        assert_eq!(report.goal_artifact, "launch-craft");
        assert_eq!(report.event_count, 6);
        assert_eq!(report.saved_chunk_count, 2);
        assert_eq!(report.frontier_count, 2);
    }

    #[test]
    fn visual_context_json_names_user_visible_shapes() {
        let output = visual_context_fixture().to_json();

        assert!(output.contains("\"schema\":\"witness.visual-context.v1\""));
        assert!(output.contains("\"motion\":\"wide unresolved field\""));
        assert!(output.contains("\"shape\":\"launch-craft\""));
        assert!(output.contains("\"shape\":\"cargo-crates\""));
        assert!(output.contains("\"semantic_owner\":\"LATTICE\""));
    }

    #[test]
    fn visual_shape_catalog_names_scenario_native_artifacts() {
        let catalog = visual_shape_catalog_fixture();

        assert_eq!(catalog.semantic_owner, "LATTICE");
        assert_eq!(catalog.shape_count, 6);
        assert!(catalog
            .shapes
            .iter()
            .any(|shape| shape.artifact_shape == "fault-circuit"));
    }

    #[test]
    fn visual_shape_catalog_json_links_chunks_to_lattice_evidence() {
        let output = visual_shape_catalog_fixture().to_json();

        assert!(output.contains("\"schema\":\"witness.visual-shapes.v1\""));
        assert!(output.contains("\"artifact_shape\":\"launch-craft\""));
        assert!(output.contains("\"artifact_shape\":\"constraint-enclosure\""));
        assert!(output.contains("\"saveable_chunk\":\"chunk:reusable-source-cargo\""));
        assert!(output.contains("\"semantic_owner\":\"LATTICE\""));
    }

    #[test]
    fn chunk_inventory_counts_reusable_and_frontier_chunks() {
        let inventory = chunk_inventory_fixture();

        assert_eq!(inventory.semantic_owner, "LATTICE");
        assert_eq!(inventory.chunk_count, 5);
        assert_eq!(inventory.reusable_count, 4);
        assert_eq!(inventory.frontier_count, 1);
    }

    #[test]
    fn chunk_inventory_json_names_saved_context_chunks() {
        let output = chunk_inventory_fixture().to_json();

        assert!(output.contains("\"schema\":\"witness.chunk-inventory.v1\""));
        assert!(output.contains("\"id\":\"chunk:launch-readiness-pack\""));
        assert!(output.contains("\"artifact_shape\":\"route-lattice\""));
        assert!(output.contains("\"reuse_status\":\"frontier-until-validated\""));
        assert!(output.contains("\"semantic_owner\":\"LATTICE\""));
    }

    #[test]
    fn build_meter_reports_context_shaping_progress() {
        let meter = build_meter_fixture();

        assert_eq!(meter.semantic_owner, "LATTICE");
        assert_eq!(meter.scenario, "product-launch-decision");
        assert_eq!(meter.overall_progress, 82);
        assert_eq!(meter.stage_count, 6);
        assert!(meter
            .stages
            .iter()
            .any(|stage| stage.id == "meter:frontier"));
    }

    #[test]
    fn build_meter_json_names_visible_progress_signals() {
        let output = build_meter_fixture().to_json();

        assert!(output.contains("\"schema\":\"witness.build-meter.v1\""));
        assert!(output.contains("\"label\":\"Evidence joined\""));
        assert!(output.contains("\"label\":\"Goal artifact formed\""));
        assert!(output.contains("\"lattice_anchor\":\"frontier:lattice-execution\""));
        assert!(output.contains("\"semantic_owner\":\"LATTICE\""));
    }

    #[test]
    fn artifact_assembly_counts_included_and_frontier_pieces() {
        let assembly = artifact_assembly_fixture();

        assert_eq!(assembly.semantic_owner, "LATTICE");
        assert_eq!(assembly.scenario, "product-launch-decision");
        assert_eq!(assembly.artifact_shape, "launch-craft");
        assert_eq!(assembly.piece_count, 6);
        assert_eq!(assembly.included_count, 5);
        assert_eq!(assembly.frontier_count, 1);
    }

    #[test]
    fn artifact_assembly_json_shows_goal_shape_with_frontier() {
        let output = artifact_assembly_fixture().to_json();

        assert!(output.contains("\"schema\":\"witness.artifact-assembly.v1\""));
        assert!(output.contains("\"artifact_shape\":\"launch-craft\""));
        assert!(output.contains("\"role\":\"go/no-go panel\""));
        assert!(output.contains("\"fit_status\":\"frontier\""));
        assert!(output.contains("\"lattice_anchor\":\"frontier:lattice-execution\""));
        assert!(output.contains("\"semantic_owner\":\"LATTICE\""));
    }

    #[test]
    fn narrowing_trace_counts_world_context_reduction() {
        let trace = narrowing_trace_fixture();

        assert_eq!(trace.semantic_owner, "LATTICE");
        assert_eq!(trace.scenario, "product-launch-decision");
        assert_eq!(trace.initial_candidate_count, 4096);
        assert_eq!(trace.final_candidate_count, 12);
        assert_eq!(trace.saved_chunk_count, 4);
        assert_eq!(trace.frontier_count, 5);
    }

    #[test]
    fn narrowing_trace_json_names_queries_chunks_and_frontier() {
        let output = narrowing_trace_fixture().to_json();

        assert!(output.contains("\"schema\":\"witness.narrowing-trace.v1\""));
        assert!(output.contains("\"query\":\"Which market signals are launch-relevant?\""));
        assert!(output.contains("\"saved_chunk\":\"chunk:policy-constraint-pack\""));
        assert!(output.contains("\"frontier\":\"frontier:lattice-execution\""));
        assert!(output.contains("\"semantic_owner\":\"LATTICE\""));
    }

    #[test]
    fn conversation_surface_keeps_chat_primary_with_visual_companion() {
        let surface = conversation_surface_fixture();

        assert_eq!(surface.semantic_owner, "LATTICE");
        assert_eq!(surface.primary_surface, "chat");
        assert_eq!(surface.companion_surface, "context-visualization");
        assert_eq!(surface.turn_count, 5);
        assert_eq!(surface.visual_update_count, 5);
        assert_eq!(surface.frontier_count, 5);
    }

    #[test]
    fn conversation_surface_json_links_turns_to_visual_updates() {
        let output = conversation_surface_fixture().to_json();

        assert!(output.contains("\"schema\":\"witness.conversation-surface.v1\""));
        assert!(output.contains("\"primary_surface\":\"chat\""));
        assert!(output.contains("\"harness_event\":\"source-pointers-added\""));
        assert!(output.contains(
            "\"visual_state\":\"launch craft forms next to the chat with frontier still visible\""
        ));
        assert!(output.contains("\"semantic_owner\":\"LATTICE\""));
    }

    #[test]
    fn operator_showcases_cover_fixed_lattice_catalog() {
        let report = operator_showcase_fixture();

        assert_eq!(report.semantic_owner, "LATTICE");
        assert!(!report.generic_dialect_system);
        assert_eq!(report.operator_count(), 21);
        for id in [
            "rank", "path", "bridge", "audit", "project", "compare", "narrow", "frontier",
            "coverage", "gaps", "diff", "refresh",
        ] {
            assert!(report.showcases.iter().any(|showcase| showcase.id == id));
        }
        assert!(report
            .showcases
            .iter()
            .all(|showcase| !showcase.dataflow.is_empty() && !showcase.edge_note.is_empty()));
    }

    #[test]
    fn operator_showcases_json_preserves_edge_boundary() {
        let output = operator_showcase_fixture().to_json();

        assert!(output.contains("\"schema\":\"witness.operator-showcases.v1\""));
        assert!(output.contains("\"generic_dialect_system\":false"));
        assert!(output.contains("\"id\":\"compare\""));
        assert!(output.contains("\"output_layer\":\"CompareReport\""));
        assert!(output.contains("fixed in the middle"));
        assert!(output.contains("not a generic projection language"));
    }

    #[test]
    fn lattice_scenario_operator_catalog_consumes_lattice_contract() {
        let report = lattice_scenario_operator_catalog_fixture();

        assert_eq!(
            report.consumed_contract_schema,
            "lattice.operator-scenario-catalog.v1"
        );
        assert_eq!(report.semantic_owner, "LATTICE");
        assert_eq!(report.edge_viewer, "WITNESS");
        assert!(!report.generic_dialect_system);
        assert_eq!(report.scenario_count(), 11);
        assert!(report.scenarios.iter().any(|scenario| {
            scenario.id == "launch" && scenario.operators == ["search", "meet", "join", "frontier"]
        }));
        assert!(report.scenarios.iter().any(|scenario| {
            scenario.id == "claim"
                && scenario.operators == ["validate", "compare", "gaps", "explain"]
        }));
        assert!(report.scenarios.iter().any(|scenario| {
            scenario.id == "reporting"
                && scenario.operators == ["search", "project", "compare", "coverage"]
        }));
    }

    #[test]
    fn lattice_scenario_operator_catalog_json_preserves_boundary() {
        let output = lattice_scenario_operator_catalog_fixture().to_json();

        assert!(output.contains("\"schema\":\"witness.lattice-scenario-operators.v1\""));
        assert!(output
            .contains("\"consumed_contract_schema\":\"lattice.operator-scenario-catalog.v1\""));
        assert!(output.contains("\"semantic_owner\":\"LATTICE\""));
        assert!(output.contains("\"edge_viewer\":\"WITNESS\""));
        assert!(output.contains("\"generic_dialect_system\":false"));
        assert!(output.contains("\"scenario_count\":11"));
        assert!(output.contains("\"operators\":[\"search\",\"meet\",\"join\",\"frontier\"]"));
        assert!(output.contains("\"operators\":[\"validate\",\"compare\",\"gaps\",\"explain\"]"));
        assert!(output.contains("\"operators\":[\"search\",\"project\",\"compare\",\"coverage\"]"));
    }
}
