//! Utility module for metrics of the type Value, with helper functions.
use std::collections::HashMap;
use log::*;
/// The struct that contains all the details for a named statistic.
/// This struct is used in [ValueStatistics.valuestatisticdetails], which holds a HashMap with the statistic name as key and this struct as value.
#[derive(Debug, Clone)]
pub struct ValueStatisticDetails {
    pub unit: String,
    pub unit_suffix: String,
    pub stat_type: String,
}
/// This struct is the main struct that provides the functionality for Value statistics.
#[derive(Debug)]
pub struct ValueStatistics {
    pub valuestatisticdetails: HashMap<String, ValueStatisticDetails>
}

impl ValueStatistics {
    /// Take a statistic name, and return the details about it.
    /// If it doesn't exist, it returns '?', and generates logging at the info level.
    pub fn lookup(
        &self,
        argument: &str
    ) -> &ValueStatisticDetails
    {
        match self.valuestatisticdetails.get(&argument.to_string())
        {
            Some(lookup) => lookup,
            None =>
            {
                info!("statistic not found! -> table.insert(\"{}\",\"?\",\"?\");", argument);
                Self::lookup(self, "?")
            },
        }
    }
    /// Create a struct holding a HashMap with all the known statistics and the specifics.
    pub fn create() -> ValueStatistics
    {
        let mut table = ValueStatistics { valuestatisticdetails: HashMap::new() };
        // special row for unknown values. Do NOT remove!
        table.insert("?", "?", "?");
        table.insert("active_background_compaction_input_bytes_added","bytes","gauge"); // 2.17.2
        table.insert("active_background_compaction_input_bytes_removed","bytes","gauge"); // 2.17.2
        table.insert("active_background_compaction_input_files_added","files","gauge"); // 2.17.2
        table.insert("active_background_compaction_input_files_removed","files","gauge"); // 2.17.2
        table.insert("active_background_compaction_tasks_added","tasks","gauge"); // 2.17.2
        table.insert("active_background_compaction_tasks_removed","tasks","gauge"); // 2.17.2
        table.insert("active_full_compaction_input_bytes_added","bytes","gauge"); // 2.17.2
        table.insert("active_full_compaction_input_bytes_removed","bytes","gauge"); // 2.17.2
        table.insert("active_full_compaction_input_files_added","files","gauge"); // 2.17.2
        table.insert("active_full_compaction_input_files_removed","files","gauge"); // 2.17.2
        table.insert("active_full_compaction_tasks_added","tasks","gauge"); // 2.17.2
        table.insert("active_full_compaction_tasks_removed","tasks","gauge"); // 2.17.2
        table.insert("active_post_split_compaction_input_bytes_added","bytes","gauge"); // 2.17.2
        table.insert("active_post_split_compaction_input_bytes_removed","bytes","gauge"); // 2.17.2
        table.insert("active_post_split_compaction_input_files_added","files","gauge"); // 2.17.2
        table.insert("active_post_split_compaction_input_files_removed","files","gauge"); // 2.17.2
        table.insert("active_post_split_compaction_tasks_added","tasks","gauge"); // 2.17.2
        table.insert("active_post_split_compaction_tasks_removed","tasks","gauge"); // 2.17.2
        table.insert("active_task_metrics_compaction_input_bytes_added", "bytes", "gauge");
        table.insert("active_task_metrics_compaction_input_bytes_removed", "bytes", "gauge");
        table.insert("active_task_metrics_compaction_input_files_added", "files", "gauge");
        table.insert("active_task_metrics_compaction_input_files_removed", "files", "gauge");
        table.insert("active_task_metrics_compaction_tasks_added", "tasks", "gauge");
        table.insert("active_task_metrics_compaction_tasks_removed", "tasks","gauge");
        table.insert("all_operations_inflight", "operations", "gauge");
        table.insert("alter_schema_operations_inflight", "operations", "gauge");
        table.insert("automatic_split_manager_time", "milliseconds", "gauge");
        table.insert("block_cache_evictions", "blocks", "counter");
        table.insert("block_cache_hits", "blocks", "counter");
        table.insert("block_cache_hits_caching", "blocks", "counter");
        table.insert("block_cache_inserts", "blocks","counter");
        table.insert("block_cache_lookups", "blocks","counter");
        table.insert("block_cache_misses", "blocks","counter");
        table.insert("block_cache_misses_caching", "blocks","counter");
        table.insert("block_cache_multi_touch_usage", "bytes","gauge");
        table.insert("block_cache_single_touch_usage", "bytes","gauge");
        table.insert("block_cache_usage", "bytes","gauge");
        table.insert("cdc_rpc_proxy_count", "requests","counter");
        table.insert("change_auto_flags_config_operations_inflight", "operations","gauge"); // 2.17
        table.insert("consistent_prefix_failed_reads", "requests", "counter");
        table.insert("consistent_prefix_read_requests", "requests","counter");
        table.insert("consistent_prefix_successful_reads", "requests","counter");
        table.insert("cpu_stime", "milliseconds","counter");
        table.insert("cpu_utime", "milliseconds","counter");
        table.insert("cql_parsers_alive", "parsers","gauge");
        table.insert("cql_parsers_created", "parsers","counter");
        table.insert("cql_processors_alive", "processors","gauge");
        table.insert("cql_processors_created", "processors","counter");
        table.insert("deadlock_detector_waiters", "transactions","gauge"); // 2.17
        table.insert("duration_ms_loading_entries_with_type_1", "milliseconds","counter");
        table.insert("duration_ms_loading_entries_with_type_10", "milliseconds","counter");
        table.insert("duration_ms_loading_entries_with_type_11", "milliseconds","counter");
        table.insert("duration_ms_loading_entries_with_type_15", "milliseconds","counter");
        table.insert("duration_ms_loading_entries_with_type_2", "milliseconds","counter");
        table.insert("duration_ms_loading_entries_with_type_3", "milliseconds","counter");
        table.insert("duration_ms_loading_entries_with_type_4", "milliseconds","counter");
        table.insert("duration_ms_loading_entries_with_type_5", "milliseconds","counter");
        table.insert("duration_ms_loading_entries_with_type_6", "milliseconds","counter");
        table.insert("duration_ms_loading_entries_with_type_7", "milliseconds","counter");
        table.insert("duration_ms_loading_entries_with_type_8", "milliseconds","counter");
        table.insert("duration_ms_loading_entries_with_type_9", "milliseconds","counter");
        table.insert("empty_operations_inflight", "operations","gauge");
        table.insert("expired_transactions", "transactions","counter");
        table.insert("follower_lag_ms", "milliseconds","gauge");
        table.insert("follower_memory_pressure_rejections", "rejections","counter");
        table.insert("generic_current_allocated_bytes", "bytes","gauge");
        table.insert("generic_heap_size", "bytes","gauge");
        table.insert("glog_error_messages", "messages","counter");
        table.insert("glog_info_messages", "messages","counter");
        table.insert("glog_warning_messages", "messages","counter");
        table.insert("history_cutoff_operations_inflight", "operations","gauge");
        table.insert("hybrid_clock_error", "microseconds","gauge");
        table.insert("hybrid_clock_hybrid_time", "microseconds","gauge");
        table.insert("hybrid_clock_skew", "microseconds","gauge");
        table.insert("in_progress_ops", "operations","gauge");
        table.insert("involuntary_context_switches", "context switches","counter");
        table.insert("iproxy_response_bytes_yb_master_MasterAdmin_AddTransactionStatusTablet", "bytes","counter");
        table.insert("is_load_balancing_enabled", "indicator", "gauge"); // 2.15.3.0
        table.insert("is_raft_leader", "indicator", "gauge");
        table.insert("leader_memory_pressure_rejections", "rejections","counter");
        table.insert("log_bytes_logged", "bytes","counter");
        table.insert("log_cache_disk_reads", "reads","counter");
        table.insert("log_cache_num_ops", "operations","gauge");
        table.insert("log_cache_size", "bytes","gauge");
        table.insert("log_gc_running", "operations","gauge");
        table.insert("log_reader_bytes_read", "bytes","counter");
        table.insert("log_reader_entries_read", "entries","counter");
        table.insert("log_wal_size", "bytes","gauge");
        table.insert("majority_done_ops", "operations","gauge");
        table.insert("majority_sst_files_rejections", "rejections","counter");
        table.insert("mem_tracker", "bytes","gauge");
        table.insert("mem_tracker_BlockBasedTable", "bytes","gauge");
        table.insert("mem_tracker_BlockBasedTable_IntentsDB", "bytes","gauge");
        table.insert("mem_tracker_BlockBasedTable_RegularDB", "bytes","gauge");
        table.insert("mem_tracker_CQL_prepared_statements", "bytes","gauge");
        table.insert("mem_tracker_CQL_processors", "bytes","gauge");
        table.insert("mem_tracker_Call", "bytes","gauge");
        table.insert("mem_tracker_Call_CQL", "bytes","gauge");
        table.insert("mem_tracker_Call_Inbound_RPC", "bytes","gauge");
        table.insert("mem_tracker_Call_Outbound_RPC", "bytes","gauge");
        table.insert("mem_tracker_Call_Redis", "bytes","gauge");
        table.insert("mem_tracker_Compressed_Read_Buffer", "bytes","gauge");
        table.insert("mem_tracker_Compressed_Read_Buffer_Receive", "bytes","gauge");
        table.insert("mem_tracker_Encrypted_Read_Buffer_Receive", "bytes","gauge");
        table.insert("mem_tracker_IntentsDB", "bytes","gauge");
        table.insert("mem_tracker_IntentsDB_MemTable", "bytes","gauge");
        table.insert("mem_tracker_OperationsFromDisk", "bytes","gauge");
        table.insert("mem_tracker_Read_Buffer", "bytes","gauge");
        table.insert("mem_tracker_Read_Buffer_CQL", "bytes","gauge");
        table.insert("mem_tracker_Read_Buffer_CQL_Reading","bytes","gauge"); // 2.17.2
        table.insert("mem_tracker_Read_Buffer_CQL_Receive","bytes","gauge"); // 2.17.2
        table.insert("mem_tracker_Read_Buffer_CQL_Sending","bytes","gauge"); // 2.17.2
        table.insert("mem_tracker_Read_Buffer_Inbound_RPC", "bytes","gauge");
        table.insert("mem_tracker_Read_Buffer_Inbound_RPC_Reading", "bytes","gauge");
        table.insert("mem_tracker_Read_Buffer_Inbound_RPC_Receive", "bytes","gauge");
        table.insert("mem_tracker_Read_Buffer_Inbound_RPC_Sending", "bytes","gauge");
        table.insert("mem_tracker_Read_Buffer_Outbound_RPC", "bytes","gauge");
        table.insert("mem_tracker_Read_Buffer_Outbound_RPC_Queueing", "bytes","gauge");
        table.insert("mem_tracker_Read_Buffer_Outbound_RPC_Reading", "bytes","gauge");
        table.insert("mem_tracker_Read_Buffer_Outbound_RPC_Receive", "bytes","gauge");
        table.insert("mem_tracker_Read_Buffer_Outbound_RPC_Sending", "bytes","gauge");
        table.insert("mem_tracker_Read_Buffer_Redis", "bytes","gauge");
        table.insert("mem_tracker_Read_Buffer_Redis_Allocated", "bytes","gauge");
        table.insert("mem_tracker_Read_Buffer_Redis_Mandatory", "bytes","gauge");
        table.insert("mem_tracker_Read_Buffer_Redis_Used", "bytes","gauge");
        table.insert("mem_tracker_RegularDB", "bytes","gauge");
        table.insert("mem_tracker_RegularDB_MemTable", "bytes","gauge");
        table.insert("mem_tracker_Tablets", "bytes","gauge");
        table.insert("mem_tracker_log_cache", "bytes","gauge");
        table.insert("mem_tracker_operation_tracker", "bytes","gauge");
        table.insert("nonactive_background_compaction_input_bytes_added","bytes","gauge"); // 2.17.2
        table.insert("nonactive_background_compaction_input_bytes_removed","bytes","gauge"); // 2.17.2
        table.insert("nonactive_background_compaction_input_files_added","files","gauge"); // 2.17.2
        table.insert("nonactive_background_compaction_input_files_removed","files","gauge"); // 2.17.2
        table.insert("nonactive_background_compaction_tasks_added","tasks","gauge"); // 2.17.2
        table.insert("nonactive_background_compaction_tasks_removed","tasks","gauge"); // 2.17.2
        table.insert("nonactive_full_compaction_input_bytes_added","bytes","gauge"); // 2.17.2
        table.insert("nonactive_full_compaction_input_bytes_removed","bytes","gauge"); // 2.17.2
        table.insert("nonactive_full_compaction_input_files_added","files","gauge"); // 2.17.2
        table.insert("nonactive_full_compaction_input_files_removed","files","gauge"); // 2.17.2
        table.insert("nonactive_full_compaction_tasks_added","tasks","gauge"); // 2.17.2
        table.insert("nonactive_full_compaction_tasks_removed","tasks","gauge"); // 2.17.2
        table.insert("nonactive_post_split_compaction_input_bytes_added","bytes","gauge"); // 2.17.2
        table.insert("nonactive_post_split_compaction_input_bytes_removed","bytes","gauge"); // 2.17.2
        table.insert("nonactive_post_split_compaction_input_files_added","files","gauge"); // 2.17.2
        table.insert("nonactive_post_split_compaction_input_files_removed","files","gauge"); // 2.17.2
        table.insert("nonactive_post_split_compaction_tasks_added","tasks","gauge"); // 2.17.2
        table.insert("nonactive_post_split_compaction_tasks_removed","tasks","gauge"); // 2.17.2
        table.insert("nonactive_task_metrics_compaction_input_bytes_added","bytes","gauge"); // 2.17.2
        table.insert("nonactive_task_metrics_compaction_input_bytes_removed","bytes","gauge"); // 2.17.2
        table.insert("nonactive_task_metrics_compaction_input_files_added","files","gauge"); // 2.17.2
        table.insert("nonactive_task_metrics_compaction_input_files_removed","files","gauge"); // 2.17.2
        table.insert("nonactive_task_metrics_compaction_tasks_added","tasks","gauge"); // 2.17.2
        table.insert("nonactive_task_metrics_compaction_tasks_removed","tasks","gauge"); // 2.17.2
        table.insert("not_leader_rejections", "rejections","counter");
        table.insert("num_entries_with_type_10_loaded", "entries","counter");
        table.insert("num_entries_with_type_11_loaded", "entries","counter");
        table.insert("num_entries_with_type_15_loaded", "entries","counter");
        table.insert("num_entries_with_type_1_loaded", "entries","counter");
        table.insert("num_entries_with_type_2_loaded", "entries","counter");
        table.insert("num_entries_with_type_3_loaded", "entries","counter");
        table.insert("num_entries_with_type_4_loaded", "entries","counter");
        table.insert("num_entries_with_type_5_loaded", "entries","counter");
        table.insert("num_entries_with_type_6_loaded", "entries","counter");
        table.insert("num_entries_with_type_6_loaded", "entries","counter");
        table.insert("num_entries_with_type_7_loaded", "entries","counter");
        table.insert("num_entries_with_type_8_loaded", "entries","counter");
        table.insert("num_entries_with_type_9_loaded", "entries","counter");
        table.insert("num_tablet_servers_dead", "entries","gauge");
        table.insert("num_tablet_servers_live", "entries","gauge");
        table.insert("operation_memory_pressure_rejections", "rejections","counter");
        table.insert("paused_background_compaction_input_bytes_added","bytes","gauge"); // 2.17.2
        table.insert("paused_background_compaction_input_bytes_removed","bytes","gauge"); // 2.17.2
        table.insert("paused_background_compaction_input_files_added","files","gauge"); // 2.17.2
        table.insert("paused_background_compaction_input_files_removed","files","gauge"); // 2.17.2
        table.insert("paused_background_compaction_tasks_added","tasks","gauge"); // 2.17.2
        table.insert("paused_background_compaction_tasks_removed","tasks","gauge"); // 2.17.2
        table.insert("paused_full_compaction_input_bytes_added","bytes","gauge"); // 2.17.2
        table.insert("paused_full_compaction_input_bytes_removed","bytes","gauge"); // 2.17.2
        table.insert("paused_full_compaction_input_files_added","files","gauge"); // 2.17.2
        table.insert("paused_full_compaction_input_files_removed","files","gauge"); // 2.17.2
        table.insert("paused_full_compaction_tasks_added","tasks","gauge"); // 2.17.2
        table.insert("paused_full_compaction_tasks_removed","tasks","gauge"); // 2.17.2
        table.insert("paused_post_split_compaction_input_bytes_added","bytes","gauge"); // 2.17.2
        table.insert("paused_post_split_compaction_input_bytes_removed","bytes","gauge"); // 2.17.2
        table.insert("paused_post_split_compaction_input_files_added","files","gauge"); // 2.17.2
        table.insert("paused_post_split_compaction_input_files_removed","files","gauge"); // 2.17.2
        table.insert("paused_post_split_compaction_tasks_added","tasks","gauge"); // 2.17.2
        table.insert("paused_post_split_compaction_tasks_removed","tasks","gauge"); // 2.17.2
        table.insert("paused_task_metrics_compaction_input_bytes_added", "bytes","gauge");
        table.insert("paused_task_metrics_compaction_input_bytes_removed", "bytes","gauge");
        table.insert("paused_task_metrics_compaction_input_files_added", "files","gauge");
        table.insert("paused_task_metrics_compaction_input_files_removed", "files","gauge");
        table.insert("paused_task_metrics_compaction_tasks_added", "tasks","gauge");
        table.insert("paused_task_metrics_compaction_tasks_removed", "tasks","gauge");
        table.insert("pg_response_cache_hits","hits","counter"); // 2.17.2
        table.insert("pg_response_cache_queries","hits","counter"); // 2.17.2
        table.insert("pgsql_consistent_prefix_read_rows", "rows","counter");
        table.insert("proxy_request_bytes_yb_consensus_ConsensusService_ChangeConfig", "bytes","counter");
        table.insert("proxy_request_bytes_yb_consensus_ConsensusService_GetConsensusState", "bytes","counter");
        table.insert("proxy_request_bytes_yb_consensus_ConsensusService_GetLastOpId", "bytes","counter");
        table.insert("proxy_request_bytes_yb_consensus_ConsensusService_GetNodeInstance", "bytes","counter");
        table.insert("proxy_request_bytes_yb_consensus_ConsensusService_LeaderElectionLost", "bytes","counter");
        table.insert("proxy_request_bytes_yb_consensus_ConsensusService_LeaderStepDown", "bytes","counter");
        table.insert("proxy_request_bytes_yb_consensus_ConsensusService_MultiRaftUpdateConsensus", "bytes","counter");
        table.insert("proxy_request_bytes_yb_consensus_ConsensusService_RequestConsensusVote", "bytes","counter");
        table.insert("proxy_request_bytes_yb_consensus_ConsensusService_RunLeaderElection", "bytes","counter");
        table.insert("proxy_request_bytes_yb_consensus_ConsensusService_StartRemoteBootstrap", "bytes","counter");
        table.insert("proxy_request_bytes_yb_consensus_ConsensusService_UnsafeChangeConfig", "bytes","counter");
        table.insert("proxy_request_bytes_yb_consensus_ConsensusService_UpdateConsensus", "bytes","counter");
        table.insert("proxy_request_bytes_yb_master_MasterAdmin_AddTransactionStatusTablet", "bytes","counter");
        table.insert("proxy_request_bytes_yb_master_MasterAdmin_CheckIfPitrActive", "bytes","counter");
        table.insert("proxy_request_bytes_yb_master_MasterAdmin_CompactSysCatalog", "bytes","counter");
        table.insert("proxy_request_bytes_yb_master_MasterAdmin_CreateTransactionStatusTable", "bytes","counter");
        table.insert("proxy_request_bytes_yb_master_MasterAdmin_DdlLog", "bytes","counter");
        table.insert("proxy_request_bytes_yb_master_MasterAdmin_DeleteNotServingTablet", "bytes","counter");
        table.insert("proxy_request_bytes_yb_master_MasterAdmin_DisableTabletSplitting", "bytes","counter");
        table.insert("proxy_request_bytes_yb_master_MasterAdmin_FlushSysCatalog", "bytes","counter");
        table.insert("proxy_request_bytes_yb_master_MasterAdmin_FlushTables", "bytes","counter");
        table.insert("proxy_request_bytes_yb_master_MasterAdmin_IsFlushTablesDone", "bytes","counter");
        table.insert("proxy_request_bytes_yb_master_MasterAdmin_IsInitDbDone", "bytes","counter");
        table.insert("proxy_request_bytes_yb_master_MasterAdmin_IsTabletSplittingComplete", "bytes","counter");
        table.insert("proxy_request_bytes_yb_master_MasterAdmin_SplitTablet", "bytes","counter");
        table.insert("proxy_request_bytes_yb_master_MasterClient_GetTableLocations", "bytes","counter");
        table.insert("proxy_request_bytes_yb_master_MasterClient_GetTabletLocations", "bytes","counter");
        table.insert("proxy_request_bytes_yb_master_MasterClient_GetTransactionStatusTablets", "bytes","counter");
        table.insert("proxy_request_bytes_yb_master_MasterClient_GetYsqlCatalogConfig", "bytes","counter");
        table.insert("proxy_request_bytes_yb_master_MasterClient_RedisConfigGet", "bytes","counter");
        table.insert("proxy_request_bytes_yb_master_MasterClient_RedisConfigSet", "bytes","counter");
        table.insert("proxy_request_bytes_yb_master_MasterClient_ReservePgsqlOids", "bytes","counter");
        table.insert("proxy_request_bytes_yb_master_MasterCluster_AreLeadersOnPreferredOnly", "bytes","counter");
        table.insert("proxy_request_bytes_yb_master_MasterCluster_ChangeLoadBalancerState", "bytes","counter");
        table.insert("proxy_request_bytes_yb_master_MasterCluster_ChangeMasterClusterConfig", "bytes","counter");
        table.insert("proxy_request_bytes_yb_master_MasterCluster_DumpState", "bytes","counter");
        table.insert("proxy_request_bytes_yb_master_MasterCluster_GetAutoFlagsConfig", "bytes","counter");
        table.insert("proxy_request_bytes_yb_master_MasterCluster_GetLeaderBlacklistCompletion", "bytes","counter");
        table.insert("proxy_request_bytes_yb_master_MasterCluster_GetLoadBalancerState", "bytes","counter");
        table.insert("proxy_request_bytes_yb_master_MasterCluster_GetLoadMoveCompletion", "bytes","counter");
        table.insert("proxy_request_bytes_yb_master_MasterCluster_GetMasterClusterConfig", "bytes","counter");
        table.insert("proxy_request_bytes_yb_master_MasterCluster_GetMasterRegistration", "bytes","counter");
        table.insert("proxy_request_bytes_yb_master_MasterCluster_IsLoadBalanced", "bytes","counter");
        table.insert("proxy_request_bytes_yb_master_MasterCluster_IsLoadBalancerIdle", "bytes","counter");
        table.insert("proxy_request_bytes_yb_master_MasterCluster_IsMasterLeaderServiceReady", "bytes","counter");
        table.insert("proxy_request_bytes_yb_master_MasterCluster_ListLiveTabletServers", "bytes","counter");
        table.insert("proxy_request_bytes_yb_master_MasterCluster_ListMasterRaftPeers", "bytes","counter");
        table.insert("proxy_request_bytes_yb_master_MasterCluster_ListMasters", "bytes","counter");
        table.insert("proxy_request_bytes_yb_master_MasterCluster_ListTabletServers", "bytes","counter");
        table.insert("proxy_request_bytes_yb_master_MasterCluster_PromoteAutoFlags", "bytes","counter"); // 2.17
        table.insert("proxy_request_bytes_yb_master_MasterCluster_RemovedMasterUpdate", "bytes","counter");
        table.insert("proxy_request_bytes_yb_master_MasterCluster_SetPreferredZones", "bytes","counter");
        table.insert("proxy_request_bytes_yb_master_MasterDcl_AlterRole", "bytes","counter");
        table.insert("proxy_request_bytes_yb_master_MasterDcl_CreateRole", "bytes","counter");
        table.insert("proxy_request_bytes_yb_master_MasterDcl_DeleteRole", "bytes","counter");
        table.insert("proxy_request_bytes_yb_master_MasterDcl_GetPermissions", "bytes","counter");
        table.insert("proxy_request_bytes_yb_master_MasterDcl_GrantRevokePermission", "bytes","counter");
        table.insert("proxy_request_bytes_yb_master_MasterDcl_GrantRevokeRole", "bytes","counter");
        table.insert("proxy_request_bytes_yb_master_MasterDdl_AlterNamespace", "bytes","counter");
        table.insert("proxy_request_bytes_yb_master_MasterDdl_AlterTable", "bytes","counter");
        table.insert("proxy_request_bytes_yb_master_MasterDdl_BackfillIndex", "bytes","counter");
        table.insert("proxy_request_bytes_yb_master_MasterDdl_CreateNamespace", "bytes","counter");
        table.insert("proxy_request_bytes_yb_master_MasterDdl_CreateTable", "bytes","counter");
        table.insert("proxy_request_bytes_yb_master_MasterDdl_CreateTablegroup", "bytes","counter");
        table.insert("proxy_request_bytes_yb_master_MasterDdl_CreateUDType", "bytes","counter");
        table.insert("proxy_request_bytes_yb_master_MasterDdl_DeleteNamespace", "bytes","counter");
        table.insert("proxy_request_bytes_yb_master_MasterDdl_DeleteTable", "bytes","counter");
        table.insert("proxy_request_bytes_yb_master_MasterDdl_DeleteTablegroup", "bytes","counter");
        table.insert("proxy_request_bytes_yb_master_MasterDdl_DeleteUDType", "bytes","counter");
        table.insert("proxy_request_bytes_yb_master_MasterDdl_GetBackfillJobs", "bytes","counter");
        table.insert("proxy_request_bytes_yb_master_MasterDdl_GetColocatedTabletSchema", "bytes","counter");
        table.insert("proxy_request_bytes_yb_master_MasterDdl_GetNamespaceInfo", "bytes","counter");
        table.insert("proxy_request_bytes_yb_master_MasterDdl_GetTableDiskSize", "bytes","counter");
        table.insert("proxy_request_bytes_yb_master_MasterDdl_GetTableSchema", "bytes","counter");
        table.insert("proxy_request_bytes_yb_master_MasterDdl_GetTablegroupSchema", "bytes","counter");
        table.insert("proxy_request_bytes_yb_master_MasterDdl_GetUDTypeInfo", "bytes","counter");
        table.insert("proxy_request_bytes_yb_master_MasterDdl_IsAlterTableDone", "bytes","counter");
        table.insert("proxy_request_bytes_yb_master_MasterDdl_IsCreateNamespaceDone", "bytes","counter");
        table.insert("proxy_request_bytes_yb_master_MasterDdl_IsCreateTableDone", "bytes","counter");
        table.insert("proxy_request_bytes_yb_master_MasterDdl_IsDeleteNamespaceDone", "bytes","counter");
        table.insert("proxy_request_bytes_yb_master_MasterDdl_IsDeleteTableDone", "bytes","counter");
        table.insert("proxy_request_bytes_yb_master_MasterDdl_IsTruncateTableDone", "bytes","counter");
        table.insert("proxy_request_bytes_yb_master_MasterDdl_LaunchBackfillIndexForTable", "bytes","counter");
        table.insert("proxy_request_bytes_yb_master_MasterDdl_ListNamespaces", "bytes","counter");
        table.insert("proxy_request_bytes_yb_master_MasterDdl_ListTablegroups", "bytes","counter");
        table.insert("proxy_request_bytes_yb_master_MasterDdl_ListTables", "bytes","counter");
        table.insert("proxy_request_bytes_yb_master_MasterDdl_ListUDTypes", "bytes","counter");
        table.insert("proxy_request_bytes_yb_master_MasterDdl_TruncateTable", "bytes","counter");
        table.insert("proxy_request_bytes_yb_master_MasterEncryption_AddUniverseKeys", "bytes","counter");
        table.insert("proxy_request_bytes_yb_master_MasterEncryption_ChangeEncryptionInfo", "bytes","counter");
        table.insert("proxy_request_bytes_yb_master_MasterEncryption_GetUniverseKeyRegistry", "bytes","counter");
        table.insert("proxy_request_bytes_yb_master_MasterEncryption_HasUniverseKeyInMemory", "bytes","counter");
        table.insert("proxy_request_bytes_yb_master_MasterEncryption_IsEncryptionEnabled", "bytes","counter");
        table.insert("proxy_request_bytes_yb_master_MasterHeartbeat_TSHeartbeat", "bytes","counter");
        table.insert("proxy_request_bytes_yb_master_MasterReplication_AlterUniverseReplication", "bytes","counter");
        table.insert("proxy_request_bytes_yb_master_MasterReplication_ChangeXClusterRole", "bytes","counter"); // 2.17
        table.insert("proxy_request_bytes_yb_master_MasterReplication_CreateCDCStream", "bytes","counter");
        table.insert("proxy_request_bytes_yb_master_MasterReplication_DeleteCDCStream", "bytes","counter");
        table.insert("proxy_request_bytes_yb_master_MasterReplication_DeleteUniverseReplication", "bytes","counter");
        table.insert("proxy_request_bytes_yb_master_MasterReplication_GetCDCDBStreamInfo", "bytes","counter");
        table.insert("proxy_request_bytes_yb_master_MasterReplication_GetCDCStream", "bytes","counter");
        table.insert("proxy_request_bytes_yb_master_MasterReplication_GetReplicationStatus", "bytes","counter"); // 2.17
        table.insert("proxy_request_bytes_yb_master_MasterReplication_GetTableSchemaFromSysCatalog","bytes","counter"); // 2.17.2
        table.insert("proxy_request_bytes_yb_master_MasterReplication_GetUDTypeMetadata", "bytes","counter");
        table.insert("proxy_request_bytes_yb_master_MasterReplication_GetUniverseReplication", "bytes","counter");
        table.insert("proxy_request_bytes_yb_master_MasterReplication_GetXClusterEstimatedDataLoss", "bytes","counter"); // 2.17
        table.insert("proxy_request_bytes_yb_master_MasterReplication_GetXClusterSafeTime", "bytes","counter"); // 2.17
        table.insert("proxy_request_bytes_yb_master_MasterReplication_IsBootstrapRequired", "bytes","counter");
        table.insert("proxy_request_bytes_yb_master_MasterReplication_IsSetupUniverseReplicationDone", "bytes","counter");
        table.insert("proxy_request_bytes_yb_master_MasterReplication_ListCDCStreams", "bytes","counter");
        table.insert("proxy_request_bytes_yb_master_MasterReplication_SetUniverseReplicationEnabled", "bytes","counter");
        table.insert("proxy_request_bytes_yb_master_MasterReplication_SetupNSUniverseReplication", "bytes","counter");
        table.insert("proxy_request_bytes_yb_master_MasterReplication_SetupUniverseReplication", "bytes","counter");
        table.insert("proxy_request_bytes_yb_master_MasterReplication_UpdateCDCStream", "bytes","counter");
        table.insert("proxy_request_bytes_yb_master_MasterReplication_UpdateConsumerOnProducerMetadata", "bytes","counter");
        table.insert("proxy_request_bytes_yb_master_MasterReplication_UpdateConsumerOnProducerSplit", "bytes","counter");
        table.insert("proxy_request_bytes_yb_master_MasterReplication_ValidateReplicationInfo", "bytes","counter");
        table.insert("proxy_request_bytes_yb_master_MasterReplication_WaitForReplicationDrain", "bytes","counter");
        table.insert("proxy_request_bytes_yb_master_MasterService_AddUniverseKeys", "bytes","counter");
        table.insert("proxy_request_bytes_yb_master_MasterService_AlterNamespace", "bytes","counter");
        table.insert("proxy_request_bytes_yb_master_MasterService_AlterRole", "bytes","counter");
        table.insert("proxy_request_bytes_yb_master_MasterService_AlterTable", "bytes","counter");
        table.insert("proxy_request_bytes_yb_master_MasterService_AlterUniverseReplication", "bytes","counter");
        table.insert("proxy_request_bytes_yb_master_MasterService_AreLoadersOnPreferredOnly", "bytes","counter");
        table.insert("proxy_request_bytes_yb_master_MasterService_BackfillIndex", "bytes","counter");
        table.insert("proxy_request_bytes_yb_master_MasterService_ChangeEncryptionInfo", "bytes","counter");
        table.insert("proxy_request_bytes_yb_master_MasterService_ChangeLoadBalancerState", "bytes","counter");
        table.insert("proxy_request_bytes_yb_master_MasterService_ChangeMasterClusterConfig", "bytes","counter");
        table.insert("proxy_request_bytes_yb_master_MasterService_CreateCDCStream", "bytes","counter");
        table.insert("proxy_request_bytes_yb_master_MasterService_CreateNamespace", "bytes","counter");
        table.insert("proxy_request_bytes_yb_master_MasterService_CreateRole", "bytes","counter");
        table.insert("proxy_request_bytes_yb_master_MasterService_CreateTable", "bytes","counter");
        table.insert("proxy_request_bytes_yb_master_MasterService_CreateTablegroup", "bytes","counter");
        table.insert("proxy_request_bytes_yb_master_MasterService_CreateTransactionStatusTable", "bytes","counter");
        table.insert("proxy_request_bytes_yb_master_MasterService_CreateUDType", "bytes","counter");
        table.insert("proxy_request_bytes_yb_master_MasterService_DdlLog", "bytes","counter");
        table.insert("proxy_request_bytes_yb_master_MasterService_DeleteCDCStream", "bytes","counter");
        table.insert("proxy_request_bytes_yb_master_MasterService_DeleteNamespace", "bytes","counter");
        table.insert("proxy_request_bytes_yb_master_MasterService_DeleteNotServingTablet", "bytes","counter");
        table.insert("proxy_request_bytes_yb_master_MasterService_DeleteRole", "bytes","counter");
        table.insert("proxy_request_bytes_yb_master_MasterService_DeleteTable", "bytes","counter");
        table.insert("proxy_request_bytes_yb_master_MasterService_DeleteTablegroup", "bytes","counter");
        table.insert("proxy_request_bytes_yb_master_MasterService_DeleteUDType", "bytes","counter");
        table.insert("proxy_request_bytes_yb_master_MasterService_DeleteUniverseReplication", "bytes","counter");
        table.insert("proxy_request_bytes_yb_master_MasterService_DumpState", "bytes","counter");
        table.insert("proxy_request_bytes_yb_master_MasterService_FlushTables", "bytes","counter");
        table.insert("proxy_request_bytes_yb_master_MasterService_GetBackfillJobs", "bytes","counter");
        table.insert("proxy_request_bytes_yb_master_MasterService_GetCDCStream", "bytes","counter");
        table.insert("proxy_request_bytes_yb_master_MasterService_GetColocatedTabletSchema", "bytes","counter");
        table.insert("proxy_request_bytes_yb_master_MasterService_GetLeaderBlacklistCompletion", "bytes","counter");
        table.insert("proxy_request_bytes_yb_master_MasterService_GetLoadBalancerState", "bytes","counter");
        table.insert("proxy_request_bytes_yb_master_MasterService_GetLoadMoveCompletion", "bytes","counter");
        table.insert("proxy_request_bytes_yb_master_MasterService_GetMasterClusterConfig", "bytes","counter");
        table.insert("proxy_request_bytes_yb_master_MasterService_GetMasterRegistration", "bytes","counter");
        table.insert("proxy_request_bytes_yb_master_MasterService_GetNamespaceInfo", "bytes","counter");
        table.insert("proxy_request_bytes_yb_master_MasterService_GetPermissions", "bytes","counter");
        table.insert("proxy_request_bytes_yb_master_MasterService_GetTableLocations", "bytes","counter");
        table.insert("proxy_request_bytes_yb_master_MasterService_GetTableSchema", "bytes","counter");
        table.insert("proxy_request_bytes_yb_master_MasterService_GetTabletLocations", "bytes","counter");
        table.insert("proxy_request_bytes_yb_master_MasterService_GetUDTypeInfo", "bytes","counter");
        table.insert("proxy_request_bytes_yb_master_MasterService_GetUniverseKeyRegistry", "bytes","counter");
        table.insert("proxy_request_bytes_yb_master_MasterService_GetUniverseReplication", "bytes","counter");
        table.insert("proxy_request_bytes_yb_master_MasterService_GetYsqlCatalogConfig", "bytes","counter");
        table.insert("proxy_request_bytes_yb_master_MasterService_GrantRevokePermission", "bytes","counter");
        table.insert("proxy_request_bytes_yb_master_MasterService_GrantRevokeRole", "bytes","counter");
        table.insert("proxy_request_bytes_yb_master_MasterService_HasUniverseKeyInMemory", "bytes","counter");
        table.insert("proxy_request_bytes_yb_master_MasterService_IsAlterTableDone", "bytes","counter");
        table.insert("proxy_request_bytes_yb_master_MasterService_IsCreateNamespaceDone", "bytes","counter");
        table.insert("proxy_request_bytes_yb_master_MasterService_IsCreateTableDone", "bytes","counter");
        table.insert("proxy_request_bytes_yb_master_MasterService_IsDeleteNamespaceDone", "bytes","counter");
        table.insert("proxy_request_bytes_yb_master_MasterService_IsDeleteTableDone", "bytes","counter");
        table.insert("proxy_request_bytes_yb_master_MasterService_IsEncryptionEnabled", "bytes","counter");
        table.insert("proxy_request_bytes_yb_master_MasterService_IsFlushTablesDone", "bytes","counter");
        table.insert("proxy_request_bytes_yb_master_MasterService_IsInitDbDone", "bytes","counter");
        table.insert("proxy_request_bytes_yb_master_MasterService_IsLoadBalanced", "bytes","counter");
        table.insert("proxy_request_bytes_yb_master_MasterService_IsLoadBalancerIdle", "bytes","counter");
        table.insert("proxy_request_bytes_yb_master_MasterService_IsMasterLeaderServiceReady", "bytes","counter");
        table.insert("proxy_request_bytes_yb_master_MasterService_IsSetupUniverseReplicationDone", "bytes","counter");
        table.insert("proxy_request_bytes_yb_master_MasterService_IsTruncateTableDone", "bytes","counter");
        table.insert("proxy_request_bytes_yb_master_MasterService_LaunchBackfillIndexForTable", "bytes","counter");
        table.insert("proxy_request_bytes_yb_master_MasterService_ListCDCStreams", "bytes","counter");
        table.insert("proxy_request_bytes_yb_master_MasterService_ListLiveTabletServers", "bytes","counter");
        table.insert("proxy_request_bytes_yb_master_MasterService_ListMasterRaftPeers", "bytes","counter");
        table.insert("proxy_request_bytes_yb_master_MasterService_ListMasters", "bytes","counter");
        table.insert("proxy_request_bytes_yb_master_MasterService_ListNamespace", "bytes","counter");
        table.insert("proxy_request_bytes_yb_master_MasterService_ListTablegroups", "bytes","counter");
        table.insert("proxy_request_bytes_yb_master_MasterService_ListTables", "bytes","counter");
        table.insert("proxy_request_bytes_yb_master_MasterService_ListTabletServers", "bytes","counter");
        table.insert("proxy_request_bytes_yb_master_MasterService_ListUDTypes", "bytes","counter");
        table.insert("proxy_request_bytes_yb_master_MasterService_RedisConfigGet", "bytes","counter");
        table.insert("proxy_request_bytes_yb_master_MasterService_RedisConfigSet", "bytes","counter");
        table.insert("proxy_request_bytes_yb_master_MasterService_RemoveMasterUpdate", "bytes","counter");
        table.insert("proxy_request_bytes_yb_master_MasterService_ReservePgsqlOids", "bytes","counter");
        table.insert("proxy_request_bytes_yb_master_MasterService_SetPreferredZone", "bytes","counter");
        table.insert("proxy_request_bytes_yb_master_MasterService_SetUniverseReplication", "bytes","counter");
        table.insert("proxy_request_bytes_yb_master_MasterService_SetupUniverseReplication", "bytes","counter");
        table.insert("proxy_request_bytes_yb_master_MasterService_SplitTablet", "bytes","counter");
        table.insert("proxy_request_bytes_yb_master_MasterService_TSHeartbeat", "bytes","counter");
        table.insert("proxy_request_bytes_yb_master_MasterService_TruncateTable", "bytes","counter");
        table.insert("proxy_request_bytes_yb_master_MasterService_UpdateCDCStream", "bytes","counter");
        table.insert("proxy_request_bytes_yb_server_GenericService_FlushCoverage", "bytes","counter");
        table.insert("proxy_request_bytes_yb_server_GenericService_GetAutoFlagsConfigVersion", "bytes","counter");
        table.insert("proxy_request_bytes_yb_server_GenericService_GetFlag", "bytes","counter");
        table.insert("proxy_request_bytes_yb_server_GenericService_GetStatus", "bytes","counter");
        table.insert("proxy_request_bytes_yb_server_GenericService_Ping", "bytes","counter");
        table.insert("proxy_request_bytes_yb_server_GenericService_RefreshFlags", "bytes","counter");
        table.insert("proxy_request_bytes_yb_server_GenericService_ReloadCertificates", "bytes","counter");
        table.insert("proxy_request_bytes_yb_server_GenericService_ServerClock", "bytes","counter");
        table.insert("proxy_request_bytes_yb_server_GenericService_SetFlag", "bytes","counter");
        table.insert("proxy_request_bytes_yb_tserver_RemoteBootstrapService_BeginRemoteBootstrapSession", "bytes","counter"); // 2.15.2.0
        table.insert("proxy_request_bytes_yb_tserver_RemoteBootstrapService_ChangePeerRole", "bytes","counter"); // 2.17.0.0
        table.insert("proxy_request_bytes_yb_tserver_RemoteBootstrapService_CheckRemoteBootstrapSessionActive", "bytes","counter");
        table.insert("proxy_request_bytes_yb_tserver_RemoteBootstrapService_EndRemoteBootstrapSession", "bytes","counter"); // 2.15.2.0
        table.insert("proxy_request_bytes_yb_tserver_RemoteBootstrapService_FetchData", "bytes","counter"); // 2.15.2.0
        table.insert("proxy_request_bytes_yb_tserver_RemoteBootstrapService_KeepLogAnchorAlive", "bytes","counter"); // 2.17.0.0
        table.insert("proxy_request_bytes_yb_tserver_RemoteBootstrapService_RegisterLogAnchor", "bytes","counter"); // 2.17.0.0
        table.insert("proxy_request_bytes_yb_tserver_RemoteBootstrapService_RemoveRemoteBootstrapSession", "bytes","counter"); // 2.17.0.0
        table.insert("proxy_request_bytes_yb_tserver_RemoteBootstrapService_RemoveSession", "bytes","counter"); // 2.15.2.0
        table.insert("proxy_request_bytes_yb_tserver_RemoteBootstrapService_UnregisterLogAnchor", "bytes","counter"); // 2.17.0.0
        table.insert("proxy_request_bytes_yb_tserver_RemoteBootstrapService_UpdateLogAnchor", "bytes","counter"); // 2.17.0.0
        table.insert("proxy_request_bytes_yb_tserver_TabletServerAdminService_AddTableToTablet", "bytes","counter");
        table.insert("proxy_request_bytes_yb_tserver_TabletServerAdminService_AlterSchema", "bytes","counter");
        table.insert("proxy_request_bytes_yb_tserver_TabletServerAdminService_BackfillDone", "bytes","counter");
        table.insert("proxy_request_bytes_yb_tserver_TabletServerAdminService_BackfillIndex", "bytes","counter");
        table.insert("proxy_request_bytes_yb_tserver_TabletServerAdminService_CopartitionTable", "bytes","counter");
        table.insert("proxy_request_bytes_yb_tserver_TabletServerAdminService_CountIntents", "bytes","counter");
        table.insert("proxy_request_bytes_yb_tserver_TabletServerAdminService_CreateTablet", "bytes","counter");
        table.insert("proxy_request_bytes_yb_tserver_TabletServerAdminService_DeleteTablet", "bytes","counter");
        table.insert("proxy_request_bytes_yb_tserver_TabletServerAdminService_FlushTablets", "bytes","counter");
        table.insert("proxy_request_bytes_yb_tserver_TabletServerAdminService_GetSafeTime", "bytes","counter");
        table.insert("proxy_request_bytes_yb_tserver_TabletServerAdminService_PrepareDeleteTransactionTablet", "bytes","counter"); // 2.17
        table.insert("proxy_request_bytes_yb_tserver_TabletServerAdminService_RemoveTableFromTablet", "bytes","counter");
        table.insert("proxy_request_bytes_yb_tserver_TabletServerAdminService_SplitTablet", "bytes","counter");
        table.insert("proxy_request_bytes_yb_tserver_TabletServerAdminService_TestRetry", "bytes","counter");
        table.insert("proxy_request_bytes_yb_tserver_TabletServerAdminService_UpdateTransactionTablesVersion", "bytes","counter"); // 2.17
        table.insert("proxy_request_bytes_yb_tserver_TabletServerAdminService_UpgradeYsql", "bytes","counter");
        table.insert("proxy_request_bytes_yb_tserver_TabletServerBackupService_TabletSnapshotOp", "bytes","counter");
        table.insert("proxy_request_bytes_yb_tserver_TabletServerService_AbortTransaction", "bytes","counter");
        table.insert("proxy_request_bytes_yb_tserver_TabletServerService_Checksum", "bytes","counter");
        table.insert("proxy_request_bytes_yb_tserver_TabletServerService_GetLogLocation", "bytes","counter");
        table.insert("proxy_request_bytes_yb_tserver_TabletServerService_GetMasterAddresses", "bytes","counter");
        table.insert("proxy_request_bytes_yb_tserver_TabletServerService_GetSharedData", "bytes","counter");
        table.insert("proxy_request_bytes_yb_tserver_TabletServerService_GetSplitKey", "bytes","counter");
        table.insert("proxy_request_bytes_yb_tserver_TabletServerService_GetTabletStatus", "bytes","counter");
        table.insert("proxy_request_bytes_yb_tserver_TabletServerService_GetTransactionStatus", "bytes","counter");
        table.insert("proxy_request_bytes_yb_tserver_TabletServerService_GetTransactionStatusAtParticipant", "bytes","counter");
        table.insert("proxy_request_bytes_yb_tserver_TabletServerService_GetTserverCatalogVersionInfo", "bytes","counter");
        table.insert("proxy_request_bytes_yb_tserver_TabletServerService_ImportData", "bytes","counter");
        table.insert("proxy_request_bytes_yb_tserver_TabletServerService_IsTabletServerReady", "bytes","counter");
        table.insert("proxy_request_bytes_yb_tserver_TabletServerService_ListMasterServers","bytes","counter"); // 2.17.2
        table.insert("proxy_request_bytes_yb_tserver_TabletServerService_ListTablets", "bytes","counter");
        table.insert("proxy_request_bytes_yb_tserver_TabletServerService_ListTabletsForTabletServer", "bytes","counter");
        table.insert("proxy_request_bytes_yb_tserver_TabletServerService_NoOp", "bytes","counter");
        table.insert("proxy_request_bytes_yb_tserver_TabletServerService_ProbeTransactionDeadlock", "bytes","counter");
        table.insert("proxy_request_bytes_yb_tserver_TabletServerService_Publish", "bytes","counter");
        table.insert("proxy_request_bytes_yb_tserver_TabletServerService_Read", "bytes","counter");
        table.insert("proxy_request_bytes_yb_tserver_TabletServerService_TakeTransaction", "bytes","counter");
        table.insert("proxy_request_bytes_yb_tserver_TabletServerService_Truncate", "bytes","counter");
        table.insert("proxy_request_bytes_yb_tserver_TabletServerService_UpdateTransaction", "bytes","counter");
        table.insert("proxy_request_bytes_yb_tserver_TabletServerService_UpdateTransactionStatusLocation", "bytes","counter");
        table.insert("proxy_request_bytes_yb_tserver_TabletServerService_UpdateTransactionWaitingForStatus", "bytes","counter");
        table.insert("proxy_request_bytes_yb_tserver_TabletServerService_VerifyTableRowRange", "bytes","counter");
        table.insert("proxy_request_bytes_yb_tserver_TabletServerService_Write", "bytes","counter");
        table.insert("proxy_response_bytes_yb_consensus_ConsensusService_ChangeConfig", "bytes","counter");
        table.insert("proxy_response_bytes_yb_consensus_ConsensusService_GetConsensusState", "bytes","counter");
        table.insert("proxy_response_bytes_yb_consensus_ConsensusService_GetLastOpId", "bytes","counter");
        table.insert("proxy_response_bytes_yb_consensus_ConsensusService_GetNodeInstance", "bytes","counter");
        table.insert("proxy_response_bytes_yb_consensus_ConsensusService_LeaderElectionLost", "bytes","counter");
        table.insert("proxy_response_bytes_yb_consensus_ConsensusService_LeaderStepDown", "bytes","counter");
        table.insert("proxy_response_bytes_yb_consensus_ConsensusService_MultiRaftUpdateConsensus", "bytes","counter");
        table.insert("proxy_response_bytes_yb_consensus_ConsensusService_RequestConsensusVote", "bytes","counter");
        table.insert("proxy_response_bytes_yb_consensus_ConsensusService_RunLeaderElection", "bytes","counter");
        table.insert("proxy_response_bytes_yb_consensus_ConsensusService_StartRemoteBootstrap", "bytes","counter");
        table.insert("proxy_response_bytes_yb_consensus_ConsensusService_UnsafeChangeConfig", "bytes","counter");
        table.insert("proxy_response_bytes_yb_consensus_ConsensusService_UpdateConsensus", "bytes","counter");
        table.insert("proxy_response_bytes_yb_master_MasterAdmin_AddTransactionStatusTablet", "bytes","counter");
        table.insert("proxy_response_bytes_yb_master_MasterAdmin_CheckIfPitrActive", "bytes","counter");
        table.insert("proxy_response_bytes_yb_master_MasterAdmin_CompactSysCatalog", "bytes","counter");
        table.insert("proxy_response_bytes_yb_master_MasterAdmin_CreateTransactionStatusTable", "bytes","counter");
        table.insert("proxy_response_bytes_yb_master_MasterAdmin_DdlLog", "bytes","counter");
        table.insert("proxy_response_bytes_yb_master_MasterAdmin_DeleteNotServingTablet", "bytes","counter");
        table.insert("proxy_response_bytes_yb_master_MasterAdmin_DisableTabletSplitting", "bytes","counter");
        table.insert("proxy_response_bytes_yb_master_MasterAdmin_FlushSysCatalog", "bytes","counter");
        table.insert("proxy_response_bytes_yb_master_MasterAdmin_FlushTables", "bytes","counter");
        table.insert("proxy_response_bytes_yb_master_MasterAdmin_IsFlushTablesDone", "bytes","counter");
        table.insert("proxy_response_bytes_yb_master_MasterAdmin_IsInitDbDone", "bytes","counter");
        table.insert("proxy_response_bytes_yb_master_MasterAdmin_IsTabletSplittingComplete", "bytes","counter");
        table.insert("proxy_response_bytes_yb_master_MasterAdmin_SplitTablet", "bytes","counter");
        table.insert("proxy_response_bytes_yb_master_MasterClient_GetTableLocations", "bytes","counter");
        table.insert("proxy_response_bytes_yb_master_MasterClient_GetTabletLocations", "bytes","counter");
        table.insert("proxy_response_bytes_yb_master_MasterClient_GetTransactionStatusTablets", "bytes","counter");
        table.insert("proxy_response_bytes_yb_master_MasterClient_GetYsqlCatalogConfig", "bytes","counter");
        table.insert("proxy_response_bytes_yb_master_MasterClient_RedisConfigGet", "bytes","counter");
        table.insert("proxy_response_bytes_yb_master_MasterClient_RedisConfigSet", "bytes","counter");
        table.insert("proxy_response_bytes_yb_master_MasterClient_ReservePgsqlOids", "bytes","counter");
        table.insert("proxy_response_bytes_yb_master_MasterCluster_AreLeadersOnPreferredOnly", "bytes","counter");
        table.insert("proxy_response_bytes_yb_master_MasterCluster_ChangeLoadBalancerState", "bytes","counter");
        table.insert("proxy_response_bytes_yb_master_MasterCluster_ChangeMasterClusterConfig", "bytes","counter");
        table.insert("proxy_response_bytes_yb_master_MasterCluster_DumpState", "bytes","counter");
        table.insert("proxy_response_bytes_yb_master_MasterCluster_GetAutoFlagsConfig", "bytes","counter");
        table.insert("proxy_response_bytes_yb_master_MasterCluster_GetLeaderBlacklistCompletion", "bytes","counter");
        table.insert("proxy_response_bytes_yb_master_MasterCluster_GetLoadBalancerState", "bytes","counter");
        table.insert("proxy_response_bytes_yb_master_MasterCluster_GetLoadMoveCompletion", "bytes","counter");
        table.insert("proxy_response_bytes_yb_master_MasterCluster_GetMasterClusterConfig", "bytes","counter");
        table.insert("proxy_response_bytes_yb_master_MasterCluster_GetMasterRegistration", "bytes","counter");
        table.insert("proxy_response_bytes_yb_master_MasterCluster_IsLoadBalanced", "bytes","counter");
        table.insert("proxy_response_bytes_yb_master_MasterCluster_IsLoadBalancerIdle", "bytes","counter");
        table.insert("proxy_response_bytes_yb_master_MasterCluster_IsMasterLeaderServiceReady", "bytes","counter");
        table.insert("proxy_response_bytes_yb_master_MasterCluster_ListLiveTabletServers", "bytes","counter");
        table.insert("proxy_response_bytes_yb_master_MasterCluster_ListMasterRaftPeers", "bytes","counter");
        table.insert("proxy_response_bytes_yb_master_MasterCluster_ListMasters", "bytes","counter");
        table.insert("proxy_response_bytes_yb_master_MasterCluster_ListTabletServers", "bytes","counter");
        table.insert("proxy_response_bytes_yb_master_MasterCluster_PromoteAutoFlags", "bytes","counter"); // 2.17
        table.insert("proxy_response_bytes_yb_master_MasterCluster_RemovedMasterUpdate", "bytes","counter");
        table.insert("proxy_response_bytes_yb_master_MasterCluster_SetPreferredZones", "bytes","counter");
        table.insert("proxy_response_bytes_yb_master_MasterDcl_AlterRole", "bytes","counter");
        table.insert("proxy_response_bytes_yb_master_MasterDcl_CreateRole", "bytes","counter");
        table.insert("proxy_response_bytes_yb_master_MasterDcl_DeleteRole", "bytes","counter");
        table.insert("proxy_response_bytes_yb_master_MasterDcl_GetPermissions", "bytes","counter");
        table.insert("proxy_response_bytes_yb_master_MasterDcl_GrantRevokePermission", "bytes","counter");
        table.insert("proxy_response_bytes_yb_master_MasterDcl_GrantRevokeRole", "bytes","counter");
        table.insert("proxy_response_bytes_yb_master_MasterDdl_AlterNamespace", "bytes","counter");
        table.insert("proxy_response_bytes_yb_master_MasterDdl_AlterTable", "bytes","counter");
        table.insert("proxy_response_bytes_yb_master_MasterDdl_BackfillIndex", "bytes","counter");
        table.insert("proxy_response_bytes_yb_master_MasterDdl_CreateNamespace", "bytes","counter");
        table.insert("proxy_response_bytes_yb_master_MasterDdl_CreateTable", "bytes","counter");
        table.insert("proxy_response_bytes_yb_master_MasterDdl_CreateTablegroup", "bytes","counter");
        table.insert("proxy_response_bytes_yb_master_MasterDdl_CreateUDType", "bytes","counter");
        table.insert("proxy_response_bytes_yb_master_MasterDdl_DeleteNamespace", "bytes","counter");
        table.insert("proxy_response_bytes_yb_master_MasterDdl_DeleteTable", "bytes","counter");
        table.insert("proxy_response_bytes_yb_master_MasterDdl_DeleteTablegroup", "bytes","counter");
        table.insert("proxy_response_bytes_yb_master_MasterDdl_DeleteUDType", "bytes","counter");
        table.insert("proxy_response_bytes_yb_master_MasterDdl_GetBackfillJobs", "bytes","counter");
        table.insert("proxy_response_bytes_yb_master_MasterDdl_GetColocatedTabletSchema", "bytes","counter");
        table.insert("proxy_response_bytes_yb_master_MasterDdl_GetNamespaceInfo", "bytes","counter");
        table.insert("proxy_response_bytes_yb_master_MasterDdl_GetTableDiskSize", "bytes","counter");
        table.insert("proxy_response_bytes_yb_master_MasterDdl_GetTableSchema", "bytes","counter");
        table.insert("proxy_response_bytes_yb_master_MasterDdl_GetTablegroupSchema", "bytes","counter");
        table.insert("proxy_response_bytes_yb_master_MasterDdl_GetUDTypeInfo", "bytes","counter");
        table.insert("proxy_response_bytes_yb_master_MasterDdl_IsAlterTableDone", "bytes","counter");
        table.insert("proxy_response_bytes_yb_master_MasterDdl_IsCreateNamespaceDone", "bytes","counter");
        table.insert("proxy_response_bytes_yb_master_MasterDdl_IsCreateTableDone", "bytes","counter");
        table.insert("proxy_response_bytes_yb_master_MasterDdl_IsDeleteNamespaceDone", "bytes","counter");
        table.insert("proxy_response_bytes_yb_master_MasterDdl_IsDeleteTableDone", "bytes","counter");
        table.insert("proxy_response_bytes_yb_master_MasterDdl_IsTruncateTableDone", "bytes","counter");
        table.insert("proxy_response_bytes_yb_master_MasterDdl_LaunchBackfillIndexForTable", "bytes","counter");
        table.insert("proxy_response_bytes_yb_master_MasterDdl_ListNamespaces", "bytes","counter");
        table.insert("proxy_response_bytes_yb_master_MasterDdl_ListTablegroups", "bytes","counter");
        table.insert("proxy_response_bytes_yb_master_MasterDdl_ListTables", "bytes","counter");
        table.insert("proxy_response_bytes_yb_master_MasterDdl_ListUDTypes", "bytes","counter");
        table.insert("proxy_response_bytes_yb_master_MasterDdl_TruncateTable", "bytes","counter");
        table.insert("proxy_response_bytes_yb_master_MasterEncryption_AddUniverseKeys", "bytes","counter");
        table.insert("proxy_response_bytes_yb_master_MasterEncryption_ChangeEncryptionInfo", "bytes","counter");
        table.insert("proxy_response_bytes_yb_master_MasterEncryption_GetUniverseKeyRegistry", "bytes","counter");
        table.insert("proxy_response_bytes_yb_master_MasterEncryption_HasUniverseKeyInMemory", "bytes","counter");
        table.insert("proxy_response_bytes_yb_master_MasterEncryption_IsEncryptionEnabled", "bytes","counter");
        table.insert("proxy_response_bytes_yb_master_MasterHeartbeat_TSHeartbeat", "bytes","counter");
        table.insert("proxy_response_bytes_yb_master_MasterReplication_AlterUniverseReplication", "bytes","counter");
        table.insert("proxy_response_bytes_yb_master_MasterReplication_ChangeXClusterRole", "bytes","counter"); // 2.17
        table.insert("proxy_response_bytes_yb_master_MasterReplication_CreateCDCStream", "bytes","counter");
        table.insert("proxy_response_bytes_yb_master_MasterReplication_DeleteCDCStream", "bytes","counter");
        table.insert("proxy_response_bytes_yb_master_MasterReplication_DeleteUniverseReplication", "bytes","counter");
        table.insert("proxy_response_bytes_yb_master_MasterReplication_GetCDCDBStreamInfo", "bytes","counter");
        table.insert("proxy_response_bytes_yb_master_MasterReplication_GetCDCStream", "bytes","counter");
        table.insert("proxy_response_bytes_yb_master_MasterReplication_GetReplicationStatus", "bytes","counter"); // 2.17
        table.insert("proxy_response_bytes_yb_master_MasterReplication_GetTableSchemaFromSysCatalog","bytes","counter"); // 2.17.2
        table.insert("proxy_response_bytes_yb_master_MasterReplication_GetUDTypeMetadata", "bytes","counter");
        table.insert("proxy_response_bytes_yb_master_MasterReplication_GetUniverseReplication", "bytes","counter");
        table.insert("proxy_response_bytes_yb_master_MasterReplication_GetXClusterEstimatedDataLoss", "bytes","counter"); // 2.17
        table.insert("proxy_response_bytes_yb_master_MasterReplication_GetXClusterSafeTime", "bytes","counter"); // 2.17
        table.insert("proxy_response_bytes_yb_master_MasterReplication_IsBootstrapRequired", "bytes","counter");
        table.insert("proxy_response_bytes_yb_master_MasterReplication_IsSetupUniverseReplicationDone", "bytes","counter");
        table.insert("proxy_response_bytes_yb_master_MasterReplication_ListCDCStreams", "bytes","counter");
        table.insert("proxy_response_bytes_yb_master_MasterReplication_SetUniverseReplicationEnabled", "bytes","counter");
        table.insert("proxy_response_bytes_yb_master_MasterReplication_SetupNSUniverseReplication", "bytes","counter");
        table.insert("proxy_response_bytes_yb_master_MasterReplication_SetupUniverseReplication", "bytes","counter");
        table.insert("proxy_response_bytes_yb_master_MasterReplication_UpdateCDCStream", "bytes","counter");
        table.insert("proxy_response_bytes_yb_master_MasterReplication_UpdateConsumerOnProducerMetadata", "bytes","counter");
        table.insert("proxy_response_bytes_yb_master_MasterReplication_UpdateConsumerOnProducerSplit", "bytes","counter");
        table.insert("proxy_response_bytes_yb_master_MasterReplication_ValidateReplicationInfo", "bytes","counter");
        table.insert("proxy_response_bytes_yb_master_MasterReplication_WaitForReplicationDrain", "bytes","counter");
        table.insert("proxy_response_bytes_yb_master_MasterService_AddUniverseKeys", "bytes","counter");
        table.insert("proxy_response_bytes_yb_master_MasterService_AlterNamespace", "bytes","counter");
        table.insert("proxy_response_bytes_yb_master_MasterService_AlterRole", "bytes","counter");
        table.insert("proxy_response_bytes_yb_master_MasterService_AlterTable", "bytes","counter");
        table.insert("proxy_response_bytes_yb_master_MasterService_AlterUniverseReplication", "bytes","counter");
        table.insert("proxy_response_bytes_yb_master_MasterService_AreLoadersOnPreferredOnly", "bytes","counter");
        table.insert("proxy_response_bytes_yb_master_MasterService_BackfillIndex", "bytes","counter");
        table.insert("proxy_response_bytes_yb_master_MasterService_ChangeEncryptionInfo", "bytes","counter");
        table.insert("proxy_response_bytes_yb_master_MasterService_ChangeLoadBalancerState", "bytes","counter");
        table.insert("proxy_response_bytes_yb_master_MasterService_ChangeMasterClusterConfig", "bytes","counter");
        table.insert("proxy_response_bytes_yb_master_MasterService_CreateCDCStream", "bytes","counter");
        table.insert("proxy_response_bytes_yb_master_MasterService_CreateNamespace", "bytes","counter");
        table.insert("proxy_response_bytes_yb_master_MasterService_CreateRole", "bytes","counter");
        table.insert("proxy_response_bytes_yb_master_MasterService_CreateTable", "bytes","counter");
        table.insert("proxy_response_bytes_yb_master_MasterService_CreateTablegroup", "bytes","counter");
        table.insert("proxy_response_bytes_yb_master_MasterService_CreateTransactionStatusTable", "bytes","counter");
        table.insert("proxy_response_bytes_yb_master_MasterService_CreateUDType", "bytes","counter");
        table.insert("proxy_response_bytes_yb_master_MasterService_DdlLog", "bytes","counter");
        table.insert("proxy_response_bytes_yb_master_MasterService_DeleteCDCStream", "bytes","counter");
        table.insert("proxy_response_bytes_yb_master_MasterService_DeleteNamespace", "bytes","counter");
        table.insert("proxy_response_bytes_yb_master_MasterService_DeleteNotServingTablet", "bytes","counter");
        table.insert("proxy_response_bytes_yb_master_MasterService_DeleteRole", "bytes","counter");
        table.insert("proxy_response_bytes_yb_master_MasterService_DeleteTable", "bytes","counter");
        table.insert("proxy_response_bytes_yb_master_MasterService_DeleteTablegroup", "bytes","counter");
        table.insert("proxy_response_bytes_yb_master_MasterService_DeleteUDType", "bytes","counter");
        table.insert("proxy_response_bytes_yb_master_MasterService_DeleteUniverseReplication", "bytes","counter");
        table.insert("proxy_response_bytes_yb_master_MasterService_DumpState", "bytes","counter");
        table.insert("proxy_response_bytes_yb_master_MasterService_FlushTables", "bytes","counter");
        table.insert("proxy_response_bytes_yb_master_MasterService_GetBackfillJobs", "bytes","counter");
        table.insert("proxy_response_bytes_yb_master_MasterService_GetCDCStream", "bytes","counter");
        table.insert("proxy_response_bytes_yb_master_MasterService_GetColocatedTabletSchema", "bytes","counter");
        table.insert("proxy_response_bytes_yb_master_MasterService_GetLeaderBlacklistCompletion", "bytes","counter");
        table.insert("proxy_response_bytes_yb_master_MasterService_GetLoadBalancerState", "bytes","counter");
        table.insert("proxy_response_bytes_yb_master_MasterService_GetLoadMoveCompletion", "bytes","counter");
        table.insert("proxy_response_bytes_yb_master_MasterService_GetMasterClusterConfig", "bytes","counter");
        table.insert("proxy_response_bytes_yb_master_MasterService_GetMasterRegistration", "bytes","counter");
        table.insert("proxy_response_bytes_yb_master_MasterService_GetNamespaceInfo", "bytes","counter");
        table.insert("proxy_response_bytes_yb_master_MasterService_GetPermissions", "bytes","counter");
        table.insert("proxy_response_bytes_yb_master_MasterService_GetTableLocations", "bytes","counter");
        table.insert("proxy_response_bytes_yb_master_MasterService_GetTableSchema", "bytes","counter");
        table.insert("proxy_response_bytes_yb_master_MasterService_GetTabletLocations", "bytes","counter");
        table.insert("proxy_response_bytes_yb_master_MasterService_GetUDTypeInfo", "bytes","counter");
        table.insert("proxy_response_bytes_yb_master_MasterService_GetUniverseKeyRegistry", "bytes","counter");
        table.insert("proxy_response_bytes_yb_master_MasterService_GetUniverseReplication", "bytes","counter");
        table.insert("proxy_response_bytes_yb_master_MasterService_GetYsqlCatalogConfig", "bytes","counter");
        table.insert("proxy_response_bytes_yb_master_MasterService_GrantRevokePermission", "bytes","counter");
        table.insert("proxy_response_bytes_yb_master_MasterService_GrantRevokeRole", "bytes","counter");
        table.insert("proxy_response_bytes_yb_master_MasterService_HasUniverseKeyInMemory", "bytes","counter");
        table.insert("proxy_response_bytes_yb_master_MasterService_IsAlterTableDone", "bytes","counter");
        table.insert("proxy_response_bytes_yb_master_MasterService_IsCreateNamespaceDone", "bytes","counter");
        table.insert("proxy_response_bytes_yb_master_MasterService_IsCreateTableDone", "bytes","counter");
        table.insert("proxy_response_bytes_yb_master_MasterService_IsDeleteNamespaceDone", "bytes","counter");
        table.insert("proxy_response_bytes_yb_master_MasterService_IsDeleteTableDone", "bytes","counter");
        table.insert("proxy_response_bytes_yb_master_MasterService_IsEncryptionEnabled", "bytes","counter");
        table.insert("proxy_response_bytes_yb_master_MasterService_IsFlushTablesDone", "bytes","counter");
        table.insert("proxy_response_bytes_yb_master_MasterService_IsInitDbDone", "bytes","counter");
        table.insert("proxy_response_bytes_yb_master_MasterService_IsLoadBalanced", "bytes","counter");
        table.insert("proxy_response_bytes_yb_master_MasterService_IsLoadBalancerIdle", "bytes","counter");
        table.insert("proxy_response_bytes_yb_master_MasterService_IsMasterLeaderServiceReady", "bytes","counter");
        table.insert("proxy_response_bytes_yb_master_MasterService_IsSetupUniverseReplicationDone", "bytes","counter");
        table.insert("proxy_response_bytes_yb_master_MasterService_IsTruncateTableDone", "bytes","counter");
        table.insert("proxy_response_bytes_yb_master_MasterService_LaunchBackfillIndexForTable", "bytes","counter");
        table.insert("proxy_response_bytes_yb_master_MasterService_ListCDCStreams", "bytes","counter");
        table.insert("proxy_response_bytes_yb_master_MasterService_ListLiveTabletServers", "bytes","counter");
        table.insert("proxy_response_bytes_yb_master_MasterService_ListMasterRaftPeers", "bytes","counter");
        table.insert("proxy_response_bytes_yb_master_MasterService_ListMasters", "bytes","counter");
        table.insert("proxy_response_bytes_yb_master_MasterService_ListNamespace", "bytes","counter");
        table.insert("proxy_response_bytes_yb_master_MasterService_ListTablegroups", "bytes","counter");
        table.insert("proxy_response_bytes_yb_master_MasterService_ListTables", "bytes","counter");
        table.insert("proxy_response_bytes_yb_master_MasterService_ListTabletServers", "bytes","counter");
        table.insert("proxy_response_bytes_yb_master_MasterService_ListUDTypes", "bytes","counter");
        table.insert("proxy_response_bytes_yb_master_MasterService_RedisConfigGet", "bytes","counter");
        table.insert("proxy_response_bytes_yb_master_MasterService_RedisConfigSet", "bytes","counter");
        table.insert("proxy_response_bytes_yb_master_MasterService_RemoveMasterUpdate", "bytes","counter");
        table.insert("proxy_response_bytes_yb_master_MasterService_ReservePgsqlOids", "bytes","counter");
        table.insert("proxy_response_bytes_yb_master_MasterService_SetPreferredZone", "bytes","counter");
        table.insert("proxy_response_bytes_yb_master_MasterService_SetUniverseReplication", "bytes","counter");
        table.insert("proxy_response_bytes_yb_master_MasterService_SetupUniverseReplication", "bytes","counter");
        table.insert("proxy_response_bytes_yb_master_MasterService_SplitTablet", "bytes","counter");
        table.insert("proxy_response_bytes_yb_master_MasterService_TSHeartbeat", "bytes","counter");
        table.insert("proxy_response_bytes_yb_master_MasterService_TruncateTable", "bytes","counter");
        table.insert("proxy_response_bytes_yb_master_MasterService_UpdateCDCStream", "bytes","counter");
        table.insert("proxy_response_bytes_yb_server_GenericService_FlushCoverage", "bytes","counter");
        table.insert("proxy_response_bytes_yb_server_GenericService_GetAutoFlagsConfigVersion", "bytes","counter");
        table.insert("proxy_response_bytes_yb_server_GenericService_GetFlag", "bytes","counter");
        table.insert("proxy_response_bytes_yb_server_GenericService_GetStatus", "bytes","counter");
        table.insert("proxy_response_bytes_yb_server_GenericService_Ping", "bytes","counter");
        table.insert("proxy_response_bytes_yb_server_GenericService_RefreshFlags", "bytes","counter");
        table.insert("proxy_response_bytes_yb_server_GenericService_ReloadCertificates", "bytes","counter");
        table.insert("proxy_response_bytes_yb_server_GenericService_ServerClock", "bytes","counter");
        table.insert("proxy_response_bytes_yb_server_GenericService_SetFlag", "bytes","counter");
        table.insert("proxy_response_bytes_yb_tserver_RemoteBootstrapService_BeginRemoteBootstrapSession", "bytes","counter"); // 2.15.2.0
        table.insert("proxy_response_bytes_yb_tserver_RemoteBootstrapService_ChangePeerRole", "bytes","counter"); // 2.17.0.0
        table.insert("proxy_response_bytes_yb_tserver_RemoteBootstrapService_CheckRemoteBootstrapSessionActive", "bytes","counter"); // 2.17.0.0
        table.insert("proxy_response_bytes_yb_tserver_RemoteBootstrapService_EndRemoteBootstrapSession", "bytes","counter"); // 2.15.2.0
        table.insert("proxy_response_bytes_yb_tserver_RemoteBootstrapService_FetchData", "bytes","counter"); // 2.15.2.0
        table.insert("proxy_response_bytes_yb_tserver_RemoteBootstrapService_KeepLogAnchorAlive", "bytes","counter"); // 2.17.0.0
        table.insert("proxy_response_bytes_yb_tserver_RemoteBootstrapService_RegisterLogAnchor", "bytes","counter"); // 2.17.0.0
        table.insert("proxy_response_bytes_yb_tserver_RemoteBootstrapService_RemoveRemoteBootstrapSession", "bytes","counter"); // 2.17.0.0
        table.insert("proxy_response_bytes_yb_tserver_RemoteBootstrapService_UnregisterLogAnchor", "bytes","counter"); // 2.17.0.0
        table.insert("proxy_response_bytes_yb_tserver_RemoteBootstrapService_UpdateLogAnchor", "bytes","counter"); // 2.17.0.0
        table.insert("proxy_response_bytes_yb_tserver_TabletServerAdminService_AddTableToTablet", "bytes","counter");
        table.insert("proxy_response_bytes_yb_tserver_TabletServerAdminService_AlterSchema", "bytes","counter");
        table.insert("proxy_response_bytes_yb_tserver_TabletServerAdminService_BackfillDone", "bytes","counter");
        table.insert("proxy_response_bytes_yb_tserver_TabletServerAdminService_BackfillIndex", "bytes","counter");
        table.insert("proxy_response_bytes_yb_tserver_TabletServerAdminService_CopartitionTable", "bytes","counter");
        table.insert("proxy_response_bytes_yb_tserver_TabletServerAdminService_CountIntents", "bytes","counter");
        table.insert("proxy_response_bytes_yb_tserver_TabletServerAdminService_CreateTablet", "bytes","counter");
        table.insert("proxy_response_bytes_yb_tserver_TabletServerAdminService_DeleteTablet", "bytes","counter");
        table.insert("proxy_response_bytes_yb_tserver_TabletServerAdminService_FlushTablets", "bytes","counter");
        table.insert("proxy_response_bytes_yb_tserver_TabletServerAdminService_GetSafeTime", "bytes","counter");
        table.insert("proxy_response_bytes_yb_tserver_TabletServerAdminService_PrepareDeleteTransactionTablet", "bytes","counter"); // 2.17
        table.insert("proxy_response_bytes_yb_tserver_TabletServerAdminService_RemoveTableFromTablet", "bytes","counter");
        table.insert("proxy_response_bytes_yb_tserver_TabletServerAdminService_SplitTablet", "bytes","counter");
        table.insert("proxy_response_bytes_yb_tserver_TabletServerAdminService_TestRetry", "bytes","counter");
        table.insert("proxy_response_bytes_yb_tserver_TabletServerAdminService_UpdateTransactionTablesVersion", "bytes","counter"); // 2.17
        table.insert("proxy_response_bytes_yb_tserver_TabletServerAdminService_UpgradeYsql", "bytes","counter");
        table.insert("proxy_response_bytes_yb_tserver_TabletServerBackupService_TabletSnapshotOp", "bytes","counter");
        table.insert("proxy_response_bytes_yb_tserver_TabletServerService_AbortTransaction", "bytes","counter");
        table.insert("proxy_response_bytes_yb_tserver_TabletServerService_Checksum", "bytes","counter");
        table.insert("proxy_response_bytes_yb_tserver_TabletServerService_GetLogLocation", "bytes","counter");
        table.insert("proxy_response_bytes_yb_tserver_TabletServerService_GetMasterAddresses", "bytes","counter");
        table.insert("proxy_response_bytes_yb_tserver_TabletServerService_GetSharedData", "bytes","counter");
        table.insert("proxy_response_bytes_yb_tserver_TabletServerService_GetSplitKey", "bytes","counter");
        table.insert("proxy_response_bytes_yb_tserver_TabletServerService_GetTabletStatus", "bytes","counter");
        table.insert("proxy_response_bytes_yb_tserver_TabletServerService_GetTransactionStatus", "bytes","counter");
        table.insert("proxy_response_bytes_yb_tserver_TabletServerService_GetTransactionStatusAtParticipant", "bytes","counter");
        table.insert("proxy_response_bytes_yb_tserver_TabletServerService_GetTserverCatalogVersionInfo", "bytes","counter");
        table.insert("proxy_response_bytes_yb_tserver_TabletServerService_ImportData", "bytes","counter");
        table.insert("proxy_response_bytes_yb_tserver_TabletServerService_IsTabletServerReady", "bytes","counter");
        table.insert("proxy_response_bytes_yb_tserver_TabletServerService_ListMasterServers","bytes","counter"); // 2.17.2
        table.insert("proxy_response_bytes_yb_tserver_TabletServerService_ListTablets", "bytes","counter");
        table.insert("proxy_response_bytes_yb_tserver_TabletServerService_ListTabletsForTabletServer", "bytes","counter");
        table.insert("proxy_response_bytes_yb_tserver_TabletServerService_NoOp", "bytes","counter");
        table.insert("proxy_response_bytes_yb_tserver_TabletServerService_ProbeTransactionDeadlock", "bytes","counter");
        table.insert("proxy_response_bytes_yb_tserver_TabletServerService_Publish", "bytes","counter");
        table.insert("proxy_response_bytes_yb_tserver_TabletServerService_Read", "bytes","counter");
        table.insert("proxy_response_bytes_yb_tserver_TabletServerService_TakeTransaction", "bytes","counter");
        table.insert("proxy_response_bytes_yb_tserver_TabletServerService_Truncate", "bytes","counter");
        table.insert("proxy_response_bytes_yb_tserver_TabletServerService_UpdateTransaction", "bytes","counter");
        table.insert("proxy_response_bytes_yb_tserver_TabletServerService_UpdateTransactionStatusLocation", "bytes","counter");
        table.insert("proxy_response_bytes_yb_tserver_TabletServerService_UpdateTransactionWaitingForStatus", "bytes","counter");
        table.insert("proxy_response_bytes_yb_tserver_TabletServerService_VerifyTableRowRange", "bytes","counter");
        table.insert("proxy_response_bytes_yb_tserver_TabletServerService_Write", "bytes","counter");
        table.insert("queued_background_compaction_input_bytes_added","bytes","gauge"); // 2.17.2
        table.insert("queued_background_compaction_input_bytes_removed","bytes","gauge"); // 2.17.2
        table.insert("queued_background_compaction_input_files_added","files","gauge"); // 2.17.2
        table.insert("queued_background_compaction_input_files_removed","files","gauge"); // 2.17.2
        table.insert("queued_background_compaction_tasks_added","tasks","gauge"); // 2.17.2
        table.insert("queued_background_compaction_tasks_removed","tasks","gauge"); // 2.17.2
        table.insert("queued_full_compaction_input_bytes_added","bytes","gauge"); // 2.17.2
        table.insert("queued_full_compaction_input_bytes_removed","bytes","gauge"); // 2.17.2
        table.insert("queued_full_compaction_input_files_added","files","gauge"); // 2.17.2
        table.insert("queued_full_compaction_input_files_removed","files","gauge"); // 2.17.2
        table.insert("queued_full_compaction_tasks_added","tasks","gauge"); // 2.17.2
        table.insert("queued_full_compaction_tasks_removed","tasks","gauge"); // 2.17.2
        table.insert("queued_post_split_compaction_input_bytes_added","bytes","gauge"); // 2.17.2
        table.insert("queued_post_split_compaction_input_bytes_removed","bytes","gauge"); // 2.17.2
        table.insert("queued_post_split_compaction_input_files_added","files","gauge"); // 2.17.2
        table.insert("queued_post_split_compaction_input_files_removed","files","gauge"); // 2.17.2
        table.insert("queued_post_split_compaction_tasks_added","tasks","gauge"); // 2.17.2
        table.insert("queued_post_split_compaction_tasks_removed","tasks","gauge"); // 2.17.2
        table.insert("queued_task_metrics_compaction_input_bytes_added", "bytes","gauge");
        table.insert("queued_task_metrics_compaction_input_bytes_removed", "bytes","gauge");
        table.insert("queued_task_metrics_compaction_input_files_added", "files","gauge");
        table.insert("queued_task_metrics_compaction_input_files_removed", "files","gauge");
        table.insert("queued_task_metrics_compaction_tasks_added", "tasks","gauge");
        table.insert("queued_task_metrics_compaction_tasks_removed", "tasks","gauge");
        table.insert("raft_term", "current consensus term","gauge");
        table.insert("replicated_retryable_request_ranges", "requests","gauge");
        table.insert("restart_read_requests", "requests","counter");
        table.insert("rocksdb_block_cache_add", "blocks","counter");
        table.insert("rocksdb_block_cache_add_failures", "blocks","counter");
        table.insert("rocksdb_block_cache_bytes_read", "bytes","counter");
        table.insert("rocksdb_block_cache_bytes_write", "bytes","counter");
        table.insert("rocksdb_block_cache_data_hit", "blocks","counter");
        table.insert("rocksdb_block_cache_data_miss", "blocks","counter");
        table.insert("rocksdb_block_cache_filter_hit", "blocks","counter");
        table.insert("rocksdb_block_cache_filter_miss", "blocks","counter");
        table.insert("rocksdb_block_cache_hit", "blocks","counter");
        table.insert("rocksdb_block_cache_index_hit", "blocks","counter");
        table.insert("rocksdb_block_cache_index_miss", "blocks","counter");
        table.insert("rocksdb_block_cache_miss", "blocks","counter");
        table.insert("rocksdb_block_cache_multi_touch_add", "blocks","counter");
        table.insert("rocksdb_block_cache_multi_touch_bytes_read", "bytes","counter");
        table.insert("rocksdb_block_cache_multi_touch_bytes_write", "bytes","counter");
        table.insert("rocksdb_block_cache_multi_touch_hit", "blocks","counter");
        table.insert("rocksdb_block_cache_single_touch_add", "blocks","counter");
        table.insert("rocksdb_block_cache_single_touch_bytes_read", "bytes","counter");
        table.insert("rocksdb_block_cache_single_touch_bytes_write", "bytes","counter");
        table.insert("rocksdb_block_cache_single_touch_hit", "blocks","counter");
        table.insert("rocksdb_block_cachecompressed_add", "blocks","counter");
        table.insert("rocksdb_block_cachecompressed_add_failures", "blocks","counter");
        table.insert("rocksdb_block_cachecompressed_hit", "blocks","counter");
        table.insert("rocksdb_block_cachecompressed_miss", "blocks","counter");
        table.insert("rocksdb_bloom_filter_checked", "blocks","counter");
        table.insert("rocksdb_bloom_filter_prefix_checked", "blocks","counter");
        table.insert("rocksdb_bloom_filter_prefix_useful", "blocks","counter");
        table.insert("rocksdb_bloom_filter_useful", "blocks","counter");
        table.insert("rocksdb_bytes_read", "bytes","counter");
        table.insert("rocksdb_bytes_written", "bytes","counter");
        table.insert("rocksdb_compact_read_bytes", "bytes","counter");
        table.insert("rocksdb_compact_write_bytes", "bytes","counter");
        table.insert("rocksdb_compaction_files_filtered", "files","counter");
        table.insert("rocksdb_compaction_files_not_filtered", "files","counter");
        table.insert("rocksdb_compaction_key_drop_new", "keys","counter");
        table.insert("rocksdb_compaction_key_drop_obsolete", "keys","counter");
        table.insert("rocksdb_compaction_key_drop_user", "keys","counter");
        table.insert("rocksdb_current_version_num_sst_files", "files","gauge");
        table.insert("rocksdb_current_version_sst_files_size", "bytes","gauge");
        table.insert("rocksdb_current_version_sst_files_uncompressed_size", "bytes","gauge");
        table.insert("rocksdb_db_iter_bytes_read", "bytes","counter");
        table.insert("rocksdb_db_mutex_wait_micros", "microseconds","counter");
        table.insert("rocksdb_filter_operation_time_nanos", "nanoseconds","counter");
        table.insert("rocksdb_flush_write_bytes", "bytes","counter");
        table.insert("rocksdb_getupdatessince_calls", "calls","counter");
        table.insert("rocksdb_l0_hit", "keys","counter");
        table.insert("rocksdb_l0_num_files_stall_micros", "microseconds","counter");
        table.insert("rocksdb_l0_slowdown_micros", "microseconds","counter");
        table.insert("rocksdb_l1_hit", "keys","counter");
        table.insert("rocksdb_l2andup_hit", "keys","counter");
        table.insert("rocksdb_memtable_compaction_micros", "microseconds","counter");
        table.insert("rocksdb_memtable_hit", "keys","counter");
        table.insert("rocksdb_memtable_miss", "keys","counter");
        table.insert("rocksdb_merge_operation_time_nanos", "nanoseconds","counter");
        table.insert("rocksdb_no_file_closes", "files","counter");
        table.insert("rocksdb_no_file_errors", "files","counter");
        table.insert("rocksdb_no_file_opens", "files","counter");
        table.insert("rocksdb_no_table_cache_iterators", "iterators","counter");
        table.insert("rocksdb_num_iterators", "iterators","counter");
        table.insert("rocksdb_number_block_not_compressed", "blocks","counter");
        table.insert("rocksdb_number_db_next", "keys","counter");
        table.insert("rocksdb_number_db_next_found", "keys","counter");
        table.insert("rocksdb_number_db_prev", "keys","counter");
        table.insert("rocksdb_number_db_prev_found", "keys","counter");
        table.insert("rocksdb_number_db_seek", "keys","counter");
        table.insert("rocksdb_number_db_seek_found", "keys","counter");
        table.insert("rocksdb_number_deletes_filtered", "deletes","counter");
        table.insert("rocksdb_number_direct_load_table_properties", "properties","counter");
        table.insert("rocksdb_number_keys_read", "keys","counter");
        table.insert("rocksdb_number_keys_updated", "keys","counter");
        table.insert("rocksdb_number_keys_written", "keys","counter");
        table.insert("rocksdb_number_merge_failures", "failures","counter");
        table.insert("rocksdb_number_multiget_bytes_read", "bytes","counter");
        table.insert("rocksdb_number_multiget_get", "calls","counter");
        table.insert("rocksdb_number_multiget_keys_read", "keys","counter");
        table.insert("rocksdb_number_reseeks_iteration", "seeks","counter");
        table.insert("rocksdb_number_superversion_acquires", "nr","counter");
        table.insert("rocksdb_number_superversion_cleanups", "nr","counter");
        table.insert("rocksdb_number_superversion_releases", "nr","counter");
        table.insert("rocksdb_rate_limit_delay_millis", "milliseconds","counter");
        table.insert("rocksdb_row_cache_hit", "rows","counter");
        table.insert("rocksdb_row_cache_miss", "rows","counter");
        table.insert("rocksdb_sequence_number", "rows","counter");
        table.insert("rocksdb_stall_micros", "microseconds","counter");
        table.insert("rocksdb_total_sst_files_size", "bytes","gauge");
        table.insert("rocksdb_wal_bytes", "bytes","counter");
        table.insert("rocksdb_wal_synced", "syncs","counter");
        table.insert("rocksdb_write_other", "writes","counter");
        table.insert("rocksdb_write_self", "writes","counter");
        table.insert("rocksdb_write_wal", "writes","counter");
        table.insert("rows_inserted", "rows","counter");
        table.insert("rpc_connections_accepted", "connections","counter");
        table.insert("rpc_connections_alive", "connections","gauge");
        table.insert("rpc_connections_created", "connections","counter");
        table.insert("rpc_inbound_calls_alive", "requests","gauge");
        table.insert("rpc_inbound_calls_created", "requests","counter");
        table.insert("rpc_outbound_calls_alive", "requests","gauge");
        table.insert("rpc_outbound_calls_created", "requests","counter");
        table.insert("rpc_timed_out_early_in_queue", "requests","counter");
        table.insert("rpc_timed_out_in_queue", "requests","counter");
        table.insert("rpcs_in_queue_yb_cdc_CDCService", "rpcs","gauge");
        table.insert("rpcs_in_queue_yb_consensus_ConsensusService", "rpcs","gauge");
        table.insert("rpcs_in_queue_yb_cqlserver_CQLServerService", "rpcs","gauge");
        table.insert("rpcs_in_queue_yb_master_MasterBackup", "rpcs","gauge");
        table.insert("rpcs_in_queue_yb_master_MasterBackupService", "rpcs","gauge");
        table.insert("rpcs_in_queue_yb_master_MasterService", "rpcs","gauge");
        table.insert("rpcs_in_queue_yb_server_GenericService", "rpcs","gauge");
        table.insert("rpcs_in_queue_yb_tserver_GenericService", "rpcs","gauge");
        table.insert("rpcs_in_queue_yb_tserver_PgClientService", "rpcs","gauge");
        table.insert("rpcs_in_queue_yb_tserver_RemoteBootstrapService", "rpcs","gauge");
        table.insert("rpcs_in_queue_yb_tserver_TabletServerAdminService", "rpcs","gauge");
        table.insert("rpcs_in_queue_yb_tserver_TabletServerBackupService", "rpcs","gauge");
        table.insert("rpcs_in_queue_yb_tserver_TabletServerService", "rpcs","gauge");
        table.insert("rpcs_queue_overflow", "requests","counter");
        table.insert("rpcs_timed_out_early_in_queue", "requests","counter");
        table.insert("rpcs_timed_out_in_queue", "requests","counter");
        table.insert("running_retryable_requests", "requests","gauge");
        table.insert("server_uptime_ms", "milliseconds","counter");
        table.insert("service_request_bytes_yb_cdc_CDCService_BootstrapProducer", "bytes","counter");
        table.insert("service_request_bytes_yb_cdc_CDCService_CheckReplicationDrain", "bytes","counter");
        table.insert("service_request_bytes_yb_cdc_CDCService_CreateCDCStream", "bytes","counter");
        table.insert("service_request_bytes_yb_cdc_CDCService_DeleteCDCStream", "bytes","counter");
        table.insert("service_request_bytes_yb_cdc_CDCService_GetCDCDBStreamInfo", "bytes","counter");
        table.insert("service_request_bytes_yb_cdc_CDCService_GetChanges", "bytes","counter");
        table.insert("service_request_bytes_yb_cdc_CDCService_GetCheckpoint", "bytes","counter");
        table.insert("service_request_bytes_yb_cdc_CDCService_GetLatestEntryOpId", "bytes","counter");
        table.insert("service_request_bytes_yb_cdc_CDCService_GetTabletListToPollForCDC", "bytes","counter"); // 2.17
        table.insert("service_request_bytes_yb_cdc_CDCService_IsBootstrapRequired", "bytes","counter");
        table.insert("service_request_bytes_yb_cdc_CDCService_ListTablets", "bytes","counter");
        table.insert("service_request_bytes_yb_cdc_CDCService_SetCDCCheckpoint", "bytes","counter");
        table.insert("service_request_bytes_yb_cdc_CDCService_UpdateCdcReplicatedIndex", "bytes","counter");
        table.insert("service_request_bytes_yb_consensus_ConsensusService_ChangeConfig", "bytes","counter");
        table.insert("service_request_bytes_yb_consensus_ConsensusService_GetConsensusState", "bytes","counter");
        table.insert("service_request_bytes_yb_consensus_ConsensusService_GetLastOpId", "bytes","counter");
        table.insert("service_request_bytes_yb_consensus_ConsensusService_GetNodeInstance", "bytes","counter");
        table.insert("service_request_bytes_yb_consensus_ConsensusService_LeaderElectionLost", "bytes","counter");
        table.insert("service_request_bytes_yb_consensus_ConsensusService_LeaderStepDown", "bytes","counter");
        table.insert("service_request_bytes_yb_consensus_ConsensusService_MultiRaftUpdateConsensus", "bytes","counter");
        table.insert("service_request_bytes_yb_consensus_ConsensusService_RequestConsensusVote", "bytes","counter");
        table.insert("service_request_bytes_yb_consensus_ConsensusService_RunLeaderElection", "bytes","counter");
        table.insert("service_request_bytes_yb_consensus_ConsensusService_StartRemoteBootstrap", "bytes","counter");
        table.insert("service_request_bytes_yb_consensus_ConsensusService_UnsafeChangeConfig", "bytes","counter");
        table.insert("service_request_bytes_yb_consensus_ConsensusService_UpdateConsensus", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterAdmin_AddTransactionStatusTablet", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterAdmin_CheckIfPitrActive", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterAdmin_CompactSysCatalog", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterAdmin_CreateTransactionStatusTable", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterAdmin_DdlLog", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterAdmin_DeleteNotServingTablet", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterAdmin_DisableTabletSplitting", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterAdmin_FlushSysCatalog", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterAdmin_FlushTables", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterAdmin_IsFlushTablesDone", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterAdmin_IsInitDbDone", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterAdmin_IsTabletSplittingComplete", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterAdmin_SplitTablet", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterBackupService_CreateSnapshot", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterBackupService_CreateSnapshotSchedule", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterBackupService_DeleteSnapshot", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterBackupService_DeleteSnapshotSchedule", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterBackupService_ImportSnapshotMeta", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterBackupService_ListSnapshotRestorations", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterBackupService_ListSnapshotSchedules", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterBackupService_ListSnapshots", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterBackupService_RestoreSnapshot", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterBackup_CreateSnapshot", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterBackup_CreateSnapshotSchedule", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterBackup_DeleteSnapshot", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterBackup_DeleteSnapshotSchedule", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterBackup_EditSnapshotSchedule", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterBackup_ImportSnapshotMeta", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterBackup_ListSnapshotRestorations", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterBackup_ListSnapshotSchedules", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterBackup_ListSnapshots", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterBackup_RestoreSnapshot", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterBackup_RestoreSnapshotSchedule", "bytes","counter"); // 2.17
        table.insert("service_request_bytes_yb_master_MasterClient_GetTableLocations", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterClient_GetTabletLocations", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterClient_GetTransactionStatusTablets", "bytes","counter"); //
        table.insert("service_request_bytes_yb_master_MasterClient_GetYsqlCatalogConfig", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterClient_RedisConfigGet", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterClient_RedisConfigSet", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterClient_ReservePgsqlOids", "bytes","counter"); //
        table.insert("service_request_bytes_yb_master_MasterCluster_AreLeadersOnPreferredOnly", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterCluster_ChangeLoadBalancerState", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterCluster_ChangeMasterClusterConfig", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterCluster_DumpState", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterCluster_GetAutoFlagsConfig", "bytes","counter"); // 2.15.2.1
        table.insert("service_request_bytes_yb_master_MasterCluster_GetLeaderBlacklistCompletion", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterCluster_GetLoadBalancerState", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterCluster_GetLoadMoveCompletion", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterCluster_GetMasterClusterConfig", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterCluster_GetMasterRegistration", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterCluster_IsLoadBalanced", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterCluster_IsLoadBalancerIdle", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterCluster_IsMasterLeaderServiceReady", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterCluster_ListLiveTabletServers", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterCluster_ListMasterRaftPeers", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterCluster_ListMasters", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterCluster_ListTabletServers", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterCluster_PromoteAutoFlags", "bytes","counter"); // 2.17
        table.insert("service_request_bytes_yb_master_MasterCluster_RemovedMasterUpdate", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterCluster_SetPreferredZones", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterDcl_AlterRole", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterDcl_CreateRole", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterDcl_DeleteRole", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterDcl_GetPermissions", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterDcl_GrantRevokePermission", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterDcl_GrantRevokeRole", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterDdl_AlterNamespace", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterDdl_AlterTable", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterDdl_BackfillIndex", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterDdl_CreateNamespace", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterDdl_CreateTable", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterDdl_CreateTablegroup", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterDdl_CreateUDType", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterDdl_DeleteNamespace", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterDdl_DeleteTable", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterDdl_DeleteTablegroup", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterDdl_DeleteUDType", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterDdl_GetBackfillJobs", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterDdl_GetColocatedTabletSchema", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterDdl_GetNamespaceInfo", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterDdl_GetTableDiskSize", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterDdl_GetTableSchema", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterDdl_GetTablegroupSchema", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterDdl_GetUDTypeInfo", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterDdl_IsAlterTableDone", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterDdl_IsCreateNamespaceDone", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterDdl_IsCreateTableDone", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterDdl_IsDeleteNamespaceDone", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterDdl_IsDeleteTableDone", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterDdl_IsTruncateTableDone", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterDdl_LaunchBackfillIndexForTable", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterDdl_ListNamespaces", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterDdl_ListTablegroups", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterDdl_ListTables", "bytes","counter"); // 2.15.3.0
        table.insert("service_request_bytes_yb_master_MasterDdl_ListUDTypes", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterDdl_TruncateTable", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterEncryption_AddUniverseKeys", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterEncryption_ChangeEncryptionInfo", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterEncryption_GetUniverseKeyRegistry", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterEncryption_HasUniverseKeyInMemory", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterEncryption_IsEncryptionEnabled", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterHeartbeat_TSHeartbeat", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterReplication_AlterUniverseReplication", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterReplication_ChangeXClusterRole", "bytes","counter"); // 2.17
        table.insert("service_request_bytes_yb_master_MasterReplication_CreateCDCStream", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterReplication_DeleteCDCStream", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterReplication_DeleteUniverseReplication", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterReplication_GetCDCDBStreamInfo", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterReplication_GetCDCStream", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterReplication_GetReplicationStatus", "bytes","counter"); // 2.17
        table.insert("service_request_bytes_yb_master_MasterReplication_GetTableSchemaFromSysCatalog","bytes","counter"); // 2.17.2
        table.insert("service_request_bytes_yb_master_MasterReplication_GetUDTypeMetadata", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterReplication_GetUniverseReplication", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterReplication_GetXClusterEstimatedDataLoss", "bytes","counter"); // 2.17
        table.insert("service_request_bytes_yb_master_MasterReplication_GetXClusterSafeTime", "bytes","counter"); // 2.17
        table.insert("service_request_bytes_yb_master_MasterReplication_IsBootstrapRequired", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterReplication_IsSetupUniverseReplicationDone", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterReplication_ListCDCStreams", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterReplication_SetUniverseReplicationEnabled", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterReplication_SetupNSUniverseReplication", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterReplication_SetupUniverseReplication", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterReplication_UpdateCDCStream", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterReplication_UpdateConsumerOnProducerMetadata", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterReplication_UpdateConsumerOnProducerSplit", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterReplication_ValidateReplicationInfo", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterReplication_WaitForReplicationDrain", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterService_AddUniverseKeys", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterService_AlterNamespace", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterService_AlterRole", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterService_AlterTable", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterService_AlterUniverseReplication", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterService_AreLoadersOnPreferredOnly", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterService_BackfillIndex", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterService_ChangeEncryptionInfo", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterService_ChangeLoadBalancerState", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterService_ChangeMasterClusterConfig", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterService_CreateCDCStream", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterService_CreateNamespace", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterService_CreateRole", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterService_CreateTable", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterService_CreateTablegroup", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterService_CreateTransactionStatusTable", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterService_CreateUDType", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterService_DdlLog", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterService_DeleteCDCStream", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterService_DeleteNamespace", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterService_DeleteNotServingTablet", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterService_DeleteRole", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterService_DeleteTable", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterService_DeleteTablegroup", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterService_DeleteUDType", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterService_DeleteUniverseReplication", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterService_DumpState", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterService_FlushTables", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterService_GetBackfillJobs", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterService_GetCDCStream", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterService_GetColocatedTabletSchema", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterService_GetLeaderBlacklistCompletion", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterService_GetLoadBalancerState", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterService_GetLoadMoveCompletion", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterService_GetMasterClusterConfig", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterService_GetMasterRegistration", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterService_GetNamespaceInfo", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterService_GetPermissions", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterService_GetTableLocations", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterService_GetTableSchema", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterService_GetTabletLocations", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterService_GetUDTypeInfo", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterService_GetUniverseKeyRegistry", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterService_GetUniverseReplication", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterService_GetYsqlCatalogConfig", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterService_GrantRevokePermission", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterService_GrantRevokeRole", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterService_HasUniverseKeyInMemory", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterService_IsAlterTableDone", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterService_IsCreateNamespaceDone", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterService_IsCreateTableDone", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterService_IsDeleteNamespaceDone", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterService_IsDeleteTableDone", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterService_IsEncryptionEnabled", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterService_IsFlushTablesDone", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterService_IsInitDbDone", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterService_IsLoadBalanced", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterService_IsLoadBalancerIdle", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterService_IsMasterLeaderServiceReady", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterService_IsSetupUniverseReplicationDone", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterService_LaunchBackfillIndexForTable", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterService_ListCDCStreams", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterService_ListLiveTabletServers", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterService_ListMasterRaftPeers", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterService_ListMasters", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterService_ListNamespaces", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterService_ListTablegroups", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterService_ListTables", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterService_ListTabletServers", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterService_ListUDType", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterService_RedisConfigGet", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterService_RedisConfigSet", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterService_RemovedMasterUpdate", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterService_ReservePgsqlOids", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterService_SetPreferredZones", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterService_SetUniverseReplication", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterService_SetupUniverseReplication", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterService_SplitTablet", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterService_TSHeartbeat", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterService_TruncateTable", "bytes","counter");
        table.insert("service_request_bytes_yb_master_MasterService_UpdateCDCStream", "bytes","counter");
        table.insert("service_request_bytes_yb_server_GenericService_FlushCoverage", "bytes","counter");
        table.insert("service_request_bytes_yb_server_GenericService_GetAutoFlagsConfigVersion", "bytes","counter");
        table.insert("service_request_bytes_yb_server_GenericService_GetFlag", "bytes","counter");
        table.insert("service_request_bytes_yb_server_GenericService_GetStatus", "bytes","counter");
        table.insert("service_request_bytes_yb_server_GenericService_Ping", "bytes","counter");
        table.insert("service_request_bytes_yb_server_GenericService_RefreshFlags", "bytes","counter");
        table.insert("service_request_bytes_yb_server_GenericService_ReloadCertificates", "bytes","counter");
        table.insert("service_request_bytes_yb_server_GenericService_ServerClock", "bytes","counter");
        table.insert("service_request_bytes_yb_server_GenericService_SetFlag", "bytes","counter");
        table.insert("service_request_bytes_yb_tserver_PgClientService_AlterDatabase", "bytes","counter");
        table.insert("service_request_bytes_yb_tserver_PgClientService_AlterTable", "bytes","counter");
        table.insert("service_request_bytes_yb_tserver_PgClientService_BackfillIndex", "bytes","counter");
        table.insert("service_request_bytes_yb_tserver_PgClientService_CheckIfPitrActive", "bytes","counter");
        table.insert("service_request_bytes_yb_tserver_PgClientService_CreateDatabase", "bytes","counter");
        table.insert("service_request_bytes_yb_tserver_PgClientService_CreateSequencesDataTable", "bytes","counter");
        table.insert("service_request_bytes_yb_tserver_PgClientService_CreateTable", "bytes","counter");
        table.insert("service_request_bytes_yb_tserver_PgClientService_CreateTablegroup", "bytes","counter");
        table.insert("service_request_bytes_yb_tserver_PgClientService_DeleteDBSequences", "bytes","counter");
        table.insert("service_request_bytes_yb_tserver_PgClientService_DeleteSequenceTuple", "bytes","counter");
        table.insert("service_request_bytes_yb_tserver_PgClientService_DropDatabase", "bytes","counter");
        table.insert("service_request_bytes_yb_tserver_PgClientService_DropTable", "bytes","counter");
        table.insert("service_request_bytes_yb_tserver_PgClientService_DropTablegroup", "bytes","counter");
        table.insert("service_request_bytes_yb_tserver_PgClientService_FinishTransaction", "bytes","counter");
        table.insert("service_request_bytes_yb_tserver_PgClientService_GetCatalogMasterVersion", "bytes","counter");
        table.insert("service_request_bytes_yb_tserver_PgClientService_GetDatabaseInfo", "bytes","counter");
        table.insert("service_request_bytes_yb_tserver_PgClientService_GetTableDiskSize", "bytes","counter");
        table.insert("service_request_bytes_yb_tserver_PgClientService_GetTserverCatalogVersionInfo", "bytes","counter"); // 2.17
        table.insert("service_request_bytes_yb_tserver_PgClientService_Heartbeat", "bytes","counter");
        table.insert("service_request_bytes_yb_tserver_PgClientService_InsertSequenceTuple", "bytes","counter");
        table.insert("service_request_bytes_yb_tserver_PgClientService_IsInitDbDone", "bytes","counter");
        table.insert("service_request_bytes_yb_tserver_PgClientService_ListLiveTabletServers", "bytes","counter");
        table.insert("service_request_bytes_yb_tserver_PgClientService_OpenTable", "bytes","counter");
        table.insert("service_request_bytes_yb_tserver_PgClientService_Perform", "bytes","counter");
        table.insert("service_request_bytes_yb_tserver_PgClientService_ReadSequenceTuple", "bytes","counter");
        table.insert("service_request_bytes_yb_tserver_PgClientService_ReserveOids", "bytes","counter");
        table.insert("service_request_bytes_yb_tserver_PgClientService_RollbackToSubTransaction", "bytes","counter");
        table.insert("service_request_bytes_yb_tserver_PgClientService_SetActiveSubTransaction", "bytes","counter");
        table.insert("service_request_bytes_yb_tserver_PgClientService_TabletServerCount", "bytes","counter");
        table.insert("service_request_bytes_yb_tserver_PgClientService_TruncateTable", "bytes","counter");
        table.insert("service_request_bytes_yb_tserver_PgClientService_UpdateSequenceTuple", "bytes","counter");
        table.insert("service_request_bytes_yb_tserver_PgClientService_ValidatePlacement", "bytes","counter");
        table.insert("service_request_bytes_yb_tserver_RemoteBootstrapService_BeginRemoteBootstrapSession", "bytes","counter");
        table.insert("service_request_bytes_yb_tserver_RemoteBootstrapService_ChangePeerRole", "bytes","counter");
        table.insert("service_request_bytes_yb_tserver_RemoteBootstrapService_CheckRemoteBootstrapSessionActive", "bytes","counter");
        table.insert("service_request_bytes_yb_tserver_RemoteBootstrapService_CheckSessionActive", "bytes","counter");
        table.insert("service_request_bytes_yb_tserver_RemoteBootstrapService_EndRemoteBootstrapSession", "bytes","counter");
        table.insert("service_request_bytes_yb_tserver_RemoteBootstrapService_FetchData", "bytes","counter");
        table.insert("service_request_bytes_yb_tserver_RemoteBootstrapService_KeepLogAnchorAlive", "bytes","counter");
        table.insert("service_request_bytes_yb_tserver_RemoteBootstrapService_RegisterLogAnchor", "bytes","counter");
        table.insert("service_request_bytes_yb_tserver_RemoteBootstrapService_RemoveRemoteBootstrapSession", "bytes","counter");
        table.insert("service_request_bytes_yb_tserver_RemoteBootstrapService_RemoveSession", "bytes","counter");
        table.insert("service_request_bytes_yb_tserver_RemoteBootstrapService_UnregisterLogAnchor", "bytes","counter");
        table.insert("service_request_bytes_yb_tserver_RemoteBootstrapService_UpdateLogAnchor", "bytes","counter");
        table.insert("service_request_bytes_yb_tserver_TabletServerAdminService_AddTableToTablet", "bytes","counter");
        table.insert("service_request_bytes_yb_tserver_TabletServerAdminService_AlterSchema", "bytes","counter");
        table.insert("service_request_bytes_yb_tserver_TabletServerAdminService_BackfillDone", "bytes","counter");
        table.insert("service_request_bytes_yb_tserver_TabletServerAdminService_BackfillIndex", "bytes","counter");
        table.insert("service_request_bytes_yb_tserver_TabletServerAdminService_CopartitionTable", "bytes","counter");
        table.insert("service_request_bytes_yb_tserver_TabletServerAdminService_CountIntents", "bytes","counter");
        table.insert("service_request_bytes_yb_tserver_TabletServerAdminService_CreateTablet", "bytes","counter");
        table.insert("service_request_bytes_yb_tserver_TabletServerAdminService_DeleteTablet", "bytes","counter");
        table.insert("service_request_bytes_yb_tserver_TabletServerAdminService_FlushTablets", "bytes","counter");
        table.insert("service_request_bytes_yb_tserver_TabletServerAdminService_GetSafeTime", "bytes","counter");
        table.insert("service_request_bytes_yb_tserver_TabletServerAdminService_PrepareDeleteTransactionTablet", "bytes","counter"); // 2.17
        table.insert("service_request_bytes_yb_tserver_TabletServerAdminService_PrepareDeleteTransactionTablet", "bytes","counter"); // 2.17
        table.insert("service_request_bytes_yb_tserver_TabletServerAdminService_RemoveTableFromTablet", "bytes","counter");
        table.insert("service_request_bytes_yb_tserver_TabletServerAdminService_SplitTablet", "bytes","counter");
        table.insert("service_request_bytes_yb_tserver_TabletServerAdminService_TabletSnapshotOp", "bytes","counter");
        table.insert("service_request_bytes_yb_tserver_TabletServerAdminService_TestRetry", "bytes","counter");
        table.insert("service_request_bytes_yb_tserver_TabletServerAdminService_UpdateConsensus", "bytes","counter");
        table.insert("service_request_bytes_yb_tserver_TabletServerAdminService_UpdateTransactionTablesVersion", "bytes","counter"); // 2.17
        table.insert("service_request_bytes_yb_tserver_TabletServerAdminService_UpgradeYsql", "bytes","counter");
        table.insert("service_request_bytes_yb_tserver_TabletServerBackupService_TabletSnapshotOp", "bytes","counter");
        table.insert("service_request_bytes_yb_tserver_TabletServerService_AbortTransaction", "bytes","counter");
        table.insert("service_request_bytes_yb_tserver_TabletServerService_Checksum", "bytes","counter");
        table.insert("service_request_bytes_yb_tserver_TabletServerService_GetLogLocation", "bytes","counter");
        table.insert("service_request_bytes_yb_tserver_TabletServerService_GetMasterAddresses", "bytes","counter");
        table.insert("service_request_bytes_yb_tserver_TabletServerService_GetSharedData", "bytes","counter");
        table.insert("service_request_bytes_yb_tserver_TabletServerService_GetSplitKey", "bytes","counter");
        table.insert("service_request_bytes_yb_tserver_TabletServerService_GetTabletStatus", "bytes","counter");
        table.insert("service_request_bytes_yb_tserver_TabletServerService_GetTransactionStatus", "bytes","counter");
        table.insert("service_request_bytes_yb_tserver_TabletServerService_GetTransactionStatusAtParticipant", "bytes","counter");
        table.insert("service_request_bytes_yb_tserver_TabletServerService_GetTserverCatalogVersionInfo", "bytes","counter");
        table.insert("service_request_bytes_yb_tserver_TabletServerService_ImportData", "bytes","counter");
        table.insert("service_request_bytes_yb_tserver_TabletServerService_IsTabletServerReady", "bytes","counter");
        table.insert("service_request_bytes_yb_tserver_TabletServerService_ListMasterServers","bytes","counter"); // 2.17.2
        table.insert("service_request_bytes_yb_tserver_TabletServerService_ListTablets", "bytes","counter");
        table.insert("service_request_bytes_yb_tserver_TabletServerService_ListTabletsForTabletServer", "bytes","counter");
        table.insert("service_request_bytes_yb_tserver_TabletServerService_NoOp", "bytes","counter");
        table.insert("service_request_bytes_yb_tserver_TabletServerService_ProbeTransactionDeadlock", "bytes","counter");
        table.insert("service_request_bytes_yb_tserver_TabletServerService_Publish", "bytes","counter");
        table.insert("service_request_bytes_yb_tserver_TabletServerService_Read", "bytes","counter");
        table.insert("service_request_bytes_yb_tserver_TabletServerService_TakeTransaction", "bytes","counter");
        table.insert("service_request_bytes_yb_tserver_TabletServerService_Truncate", "bytes","counter");
        table.insert("service_request_bytes_yb_tserver_TabletServerService_UpdateTransaction", "bytes","counter");
        table.insert("service_request_bytes_yb_tserver_TabletServerService_UpdateTransactionStatusLocation", "bytes","counter");
        table.insert("service_request_bytes_yb_tserver_TabletServerService_UpdateTransactionWaitingForStatus", "bytes","counter");
        table.insert("service_request_bytes_yb_tserver_TabletServerService_VerifyTableRowRange", "bytes","counter");
        table.insert("service_request_bytes_yb_tserver_TabletServerService_Write", "bytes","counter");
        table.insert("service_response_bytes_yb_cdc_CDCService_BootstrapProducer", "bytes","counter");
        table.insert("service_response_bytes_yb_cdc_CDCService_CheckReplicationDrain", "bytes","counter");
        table.insert("service_response_bytes_yb_cdc_CDCService_CreateCDCStream", "bytes","counter");
        table.insert("service_response_bytes_yb_cdc_CDCService_DeleteCDCStream", "bytes","counter");
        table.insert("service_response_bytes_yb_cdc_CDCService_GetCDCDBStreamInfo", "bytes","counter");
        table.insert("service_response_bytes_yb_cdc_CDCService_GetChanges", "bytes","counter");
        table.insert("service_response_bytes_yb_cdc_CDCService_GetCheckpoint", "bytes","counter");
        table.insert("service_response_bytes_yb_cdc_CDCService_GetLatestEntryOpId", "bytes","counter");
        table.insert("service_response_bytes_yb_cdc_CDCService_GetTabletListToPollForCDC", "bytes","counter"); // 2.17
        table.insert("service_response_bytes_yb_cdc_CDCService_IsBootstrapRequired", "bytes","counter");
        table.insert("service_response_bytes_yb_cdc_CDCService_ListTablets", "bytes","counter");
        table.insert("service_response_bytes_yb_cdc_CDCService_SetCDCCheckpoint", "bytes","counter");
        table.insert("service_response_bytes_yb_cdc_CDCService_UpdateCdcReplicatedIndex", "bytes","counter");
        table.insert("service_response_bytes_yb_consensus_ConsensusService_ChangeConfig", "bytes","counter");
        table.insert("service_response_bytes_yb_consensus_ConsensusService_GetConsensusState", "bytes","counter");
        table.insert("service_response_bytes_yb_consensus_ConsensusService_GetLastOpId", "bytes","counter");
        table.insert("service_response_bytes_yb_consensus_ConsensusService_GetNodeInstance", "bytes","counter");
        table.insert("service_response_bytes_yb_consensus_ConsensusService_LeaderElectionLost", "bytes","counter");
        table.insert("service_response_bytes_yb_consensus_ConsensusService_LeaderStepDown", "bytes","counter");
        table.insert("service_response_bytes_yb_consensus_ConsensusService_MultiRaftUpdateConsensus", "bytes","counter");
        table.insert("service_response_bytes_yb_consensus_ConsensusService_RequestConsensusVote", "bytes","counter");
        table.insert("service_response_bytes_yb_consensus_ConsensusService_RunLeaderElection", "bytes","counter");
        table.insert("service_response_bytes_yb_consensus_ConsensusService_StartRemoteBootstrap", "bytes","counter");
        table.insert("service_response_bytes_yb_consensus_ConsensusService_UnsafeChangeConfig", "bytes","counter");
        table.insert("service_response_bytes_yb_consensus_ConsensusService_UpdateConsensus", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterAdmin_AddTransactionStatusTablet", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterAdmin_CheckIfPitrActive", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterAdmin_CompactSysCatalog", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterAdmin_CreateTransactionStatusTable", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterAdmin_DdlLog", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterAdmin_DeleteNotServingTablet", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterAdmin_DisableTabletSplitting", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterAdmin_FlushSysCatalog", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterAdmin_FlushTables", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterAdmin_IsFlushTablesDone", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterAdmin_IsInitDbDone", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterAdmin_IsTabletSplittingComplete", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterAdmin_SplitTablet", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterBackupService_CreateSnapshot", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterBackupService_CreateSnapshotSchedule", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterBackupService_DeleteSnapshot", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterBackupService_DeleteSnapshotSchedule", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterBackupService_ImportSnapshotMeta", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterBackupService_ListSnapshotRestorations", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterBackupService_ListSnapshotSchedules", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterBackupService_ListSnapshots", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterBackupService_RestoreSnapshot", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterBackup_CreateSnapshot", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterBackup_CreateSnapshotSchedule", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterBackup_DeleteSnapshot", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterBackup_DeleteSnapshotSchedule", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterBackup_EditSnapshotSchedule", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterBackup_ImportSnapshotMeta", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterBackup_ListSnapshotRestorations", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterBackup_ListSnapshotSchedules", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterBackup_ListSnapshots", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterBackup_RestoreSnapshot", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterBackup_RestoreSnapshotSchedule", "bytes","counter"); // 2.17
        table.insert("service_response_bytes_yb_master_MasterClient_GetTableLocations", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterClient_GetTabletLocations", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterClient_GetTransactionStatusTablets", "bytes","counter"); //
        table.insert("service_response_bytes_yb_master_MasterClient_GetYsqlCatalogConfig", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterClient_RedisConfigGet", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterClient_RedisConfigSet", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterClient_ReservePgsqlOids", "bytes","counter"); //
        table.insert("service_response_bytes_yb_master_MasterCluster_AreLeadersOnPreferredOnly", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterCluster_ChangeLoadBalancerState", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterCluster_ChangeMasterClusterConfig", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterCluster_DumpState", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterCluster_GetAutoFlagsConfig", "bytes","counter"); // 2.15.2.1
        table.insert("service_response_bytes_yb_master_MasterCluster_GetLeaderBlacklistCompletion", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterCluster_GetLoadBalancerState", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterCluster_GetLoadMoveCompletion", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterCluster_GetMasterClusterConfig", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterCluster_GetMasterRegistration", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterCluster_IsLoadBalanced", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterCluster_IsLoadBalancerIdle", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterCluster_IsMasterLeaderServiceReady", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterCluster_ListLiveTabletServers", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterCluster_ListMasterRaftPeers", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterCluster_ListMasters", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterCluster_ListTabletServers", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterCluster_PromoteAutoFlags", "bytes","counter"); // 2.17
        table.insert("service_response_bytes_yb_master_MasterCluster_RemovedMasterUpdate", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterCluster_SetPreferredZones", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterDcl_AlterRole", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterDcl_CreateRole", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterDcl_DeleteRole", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterDcl_GetPermissions", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterDcl_GrantRevokePermission", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterDcl_GrantRevokeRole", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterDdl_AlterNamespace", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterDdl_AlterTable", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterDdl_BackfillIndex", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterDdl_CreateNamespace", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterDdl_CreateTable", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterDdl_CreateTablegroup", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterDdl_CreateUDType", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterDdl_DeleteNamespace", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterDdl_DeleteTable", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterDdl_DeleteTablegroup", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterDdl_DeleteUDType", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterDdl_GetBackfillJobs", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterDdl_GetColocatedTabletSchema", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterDdl_GetNamespaceInfo", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterDdl_GetTableDiskSize", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterDdl_GetTableSchema", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterDdl_GetTablegroupSchema", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterDdl_GetUDTypeInfo", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterDdl_IsAlterTableDone", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterDdl_IsCreateNamespaceDone", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterDdl_IsCreateTableDone", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterDdl_IsDeleteNamespaceDone", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterDdl_IsDeleteTableDone", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterDdl_IsTruncateTableDone", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterDdl_LaunchBackfillIndexForTable", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterDdl_ListNamespaces", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterDdl_ListTablegroups", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterDdl_ListTables", "bytes","counter"); // 2.15.3.0
        table.insert("service_response_bytes_yb_master_MasterDdl_ListUDTypes", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterDdl_TruncateTable", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterEncryption_AddUniverseKeys", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterEncryption_ChangeEncryptionInfo", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterEncryption_GetUniverseKeyRegistry", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterEncryption_HasUniverseKeyInMemory", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterEncryption_IsEncryptionEnabled", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterHeartbeat_TSHeartbeat", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterReplication_AlterUniverseReplication", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterReplication_ChangeXClusterRole", "bytes","counter"); // 2.17
        table.insert("service_response_bytes_yb_master_MasterReplication_CreateCDCStream", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterReplication_DeleteCDCStream", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterReplication_DeleteUniverseReplication", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterReplication_GetCDCDBStreamInfo", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterReplication_GetCDCStream", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterReplication_GetReplicationStatus", "bytes","counter"); // 2.17
        table.insert("service_response_bytes_yb_master_MasterReplication_GetTableSchemaFromSysCatalog","bytes","counter"); // 2.17.2
        table.insert("service_response_bytes_yb_master_MasterReplication_GetUDTypeMetadata", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterReplication_GetUniverseReplication", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterReplication_GetXClusterEstimatedDataLoss", "bytes","counter"); // 2.17
        table.insert("service_response_bytes_yb_master_MasterReplication_GetXClusterSafeTime", "bytes","counter"); // 2.17
        table.insert("service_response_bytes_yb_master_MasterReplication_IsBootstrapRequired", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterReplication_IsSetupUniverseReplicationDone", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterReplication_ListCDCStreams", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterReplication_SetUniverseReplicationEnabled", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterReplication_SetupNSUniverseReplication", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterReplication_SetupUniverseReplication", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterReplication_UpdateCDCStream", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterReplication_UpdateConsumerOnProducerMetadata", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterReplication_UpdateConsumerOnProducerSplit", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterReplication_ValidateReplicationInfo", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterReplication_WaitForReplicationDrain", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterService_AddUniverseKeys", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterService_AlterNamespace", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterService_AlterRole", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterService_AlterTable", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterService_AlterUniverseReplication", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterService_AreLoadersOnPreferredOnly", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterService_BackfillIndex", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterService_ChangeEncryptionInfo", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterService_ChangeLoadBalancerState", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterService_ChangeMasterClusterConfig", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterService_CreateCDCStream", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterService_CreateNamespace", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterService_CreateRole", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterService_CreateTable", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterService_CreateTablegroup", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterService_CreateTransactionStatusTable", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterService_CreateUDType", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterService_DdlLog", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterService_DeleteCDCStream", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterService_DeleteNamespace", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterService_DeleteNotServingTablet", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterService_DeleteRole", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterService_DeleteTable", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterService_DeleteTablegroup", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterService_DeleteUDType", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterService_DeleteUniverseReplication", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterService_DumpState", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterService_FlushTables", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterService_GetBackfillJobs", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterService_GetCDCStream", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterService_GetColocatedTabletSchema", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterService_GetLeaderBlacklistCompletion", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterService_GetLoadBalancerState", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterService_GetLoadMoveCompletion", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterService_GetMasterClusterConfig", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterService_GetMasterRegistration", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterService_GetNamespaceInfo", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterService_GetPermissions", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterService_GetTableLocations", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterService_GetTableSchema", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterService_GetTabletLocations", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterService_GetUDTypeInfo", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterService_GetUniverseKeyRegistry", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterService_GetUniverseReplication", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterService_GetYsqlCatalogConfig", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterService_GrantRevokePermission", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterService_GrantRevokeRole", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterService_HasUniverseKeyInMemory", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterService_IsAlterTableDone", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterService_IsCreateNamespaceDone", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterService_IsCreateTableDone", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterService_IsDeleteNamespaceDone", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterService_IsDeleteTableDone", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterService_IsEncryptionEnabled", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterService_IsFlushTablesDone", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterService_IsInitDbDone", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterService_IsLoadBalanced", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterService_IsLoadBalancerIdle", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterService_IsMasterLeaderServiceReady", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterService_IsSetupUniverseReplicationDone", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterService_LaunchBackfillIndexForTable", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterService_ListCDCStreams", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterService_ListLiveTabletServers", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterService_ListMasterRaftPeers", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterService_ListMasters", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterService_ListNamespaces", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterService_ListTablegroups", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterService_ListTables", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterService_ListTabletServers", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterService_ListUDType", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterService_RedisConfigGet", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterService_RedisConfigSet", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterService_RemovedMasterUpdate", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterService_ReservePgsqlOids", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterService_SetPreferredZones", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterService_SetUniverseReplication", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterService_SetupUniverseReplication", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterService_SplitTablet", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterService_TSHeartbeat", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterService_TruncateTable", "bytes","counter");
        table.insert("service_response_bytes_yb_master_MasterService_UpdateCDCStream", "bytes","counter");
        table.insert("service_response_bytes_yb_server_GenericService_FlushCoverage", "bytes","counter");
        table.insert("service_response_bytes_yb_server_GenericService_GetAutoFlagsConfigVersion", "bytes","counter");
        table.insert("service_response_bytes_yb_server_GenericService_GetFlag", "bytes","counter");
        table.insert("service_response_bytes_yb_server_GenericService_GetStatus", "bytes","counter");
        table.insert("service_response_bytes_yb_server_GenericService_Ping", "bytes","counter");
        table.insert("service_response_bytes_yb_server_GenericService_RefreshFlags", "bytes","counter");
        table.insert("service_response_bytes_yb_server_GenericService_ReloadCertificates", "bytes","counter");
        table.insert("service_response_bytes_yb_server_GenericService_ServerClock", "bytes","counter");
        table.insert("service_response_bytes_yb_server_GenericService_SetFlag", "bytes","counter");
        table.insert("service_response_bytes_yb_tserver_PgClientService_AlterDatabase", "bytes","counter");
        table.insert("service_response_bytes_yb_tserver_PgClientService_AlterTable", "bytes","counter");
        table.insert("service_response_bytes_yb_tserver_PgClientService_BackfillIndex", "bytes","counter");
        table.insert("service_response_bytes_yb_tserver_PgClientService_CheckIfPitrActive", "bytes","counter");
        table.insert("service_response_bytes_yb_tserver_PgClientService_CreateDatabase", "bytes","counter");
        table.insert("service_response_bytes_yb_tserver_PgClientService_CreateSequencesDataTable", "bytes","counter");
        table.insert("service_response_bytes_yb_tserver_PgClientService_CreateTable", "bytes","counter");
        table.insert("service_response_bytes_yb_tserver_PgClientService_CreateTablegroup", "bytes","counter");
        table.insert("service_response_bytes_yb_tserver_PgClientService_DeleteDBSequences", "bytes","counter");
        table.insert("service_response_bytes_yb_tserver_PgClientService_DeleteSequenceTuple", "bytes","counter");
        table.insert("service_response_bytes_yb_tserver_PgClientService_DropDatabase", "bytes","counter");
        table.insert("service_response_bytes_yb_tserver_PgClientService_DropTable", "bytes","counter");
        table.insert("service_response_bytes_yb_tserver_PgClientService_DropTablegroup", "bytes","counter");
        table.insert("service_response_bytes_yb_tserver_PgClientService_FinishTransaction", "bytes","counter");
        table.insert("service_response_bytes_yb_tserver_PgClientService_GetCatalogMasterVersion", "bytes","counter");
        table.insert("service_response_bytes_yb_tserver_PgClientService_GetDatabaseInfo", "bytes","counter");
        table.insert("service_response_bytes_yb_tserver_PgClientService_GetTableDiskSize", "bytes","counter");
        table.insert("service_response_bytes_yb_tserver_PgClientService_GetTserverCatalogVersionInfo", "bytes","counter"); // 2.17
        table.insert("service_response_bytes_yb_tserver_PgClientService_Heartbeat", "bytes","counter");
        table.insert("service_response_bytes_yb_tserver_PgClientService_InsertSequenceTuple", "bytes","counter");
        table.insert("service_response_bytes_yb_tserver_PgClientService_IsInitDbDone", "bytes","counter");
        table.insert("service_response_bytes_yb_tserver_PgClientService_ListLiveTabletServers", "bytes","counter");
        table.insert("service_response_bytes_yb_tserver_PgClientService_OpenTable", "bytes","counter");
        table.insert("service_response_bytes_yb_tserver_PgClientService_Perform", "bytes","counter");
        table.insert("service_response_bytes_yb_tserver_PgClientService_ReadSequenceTuple", "bytes","counter");
        table.insert("service_response_bytes_yb_tserver_PgClientService_ReserveOids", "bytes","counter");
        table.insert("service_response_bytes_yb_tserver_PgClientService_RollbackToSubTransaction", "bytes","counter");
        table.insert("service_response_bytes_yb_tserver_PgClientService_SetActiveSubTransaction", "bytes","counter");
        table.insert("service_response_bytes_yb_tserver_PgClientService_TabletServerCount", "bytes","counter");
        table.insert("service_response_bytes_yb_tserver_PgClientService_TruncateTable", "bytes","counter");
        table.insert("service_response_bytes_yb_tserver_PgClientService_UpdateSequenceTuple", "bytes","counter");
        table.insert("service_response_bytes_yb_tserver_PgClientService_ValidatePlacement", "bytes","counter");
        table.insert("service_response_bytes_yb_tserver_RemoteBootstrapService_BeginRemoteBootstrapSession", "bytes","counter");
        table.insert("service_response_bytes_yb_tserver_RemoteBootstrapService_ChangePeerRole", "bytes","counter");
        table.insert("service_response_bytes_yb_tserver_RemoteBootstrapService_CheckRemoteBootstrapSessionActive", "bytes","counter");
        table.insert("service_response_bytes_yb_tserver_RemoteBootstrapService_CheckSessionActive", "bytes","counter");
        table.insert("service_response_bytes_yb_tserver_RemoteBootstrapService_EndRemoteBootstrapSession", "bytes","counter");
        table.insert("service_response_bytes_yb_tserver_RemoteBootstrapService_FetchData", "bytes","counter");
        table.insert("service_response_bytes_yb_tserver_RemoteBootstrapService_KeepLogAnchorAlive", "bytes","counter");
        table.insert("service_response_bytes_yb_tserver_RemoteBootstrapService_RegisterLogAnchor", "bytes","counter");
        table.insert("service_response_bytes_yb_tserver_RemoteBootstrapService_RemoveRemoteBootstrapSession", "bytes","counter");
        table.insert("service_response_bytes_yb_tserver_RemoteBootstrapService_RemoveSession", "bytes","counter");
        table.insert("service_response_bytes_yb_tserver_RemoteBootstrapService_UnregisterLogAnchor", "bytes","counter");
        table.insert("service_response_bytes_yb_tserver_RemoteBootstrapService_UpdateLogAnchor", "bytes","counter");
        table.insert("service_response_bytes_yb_tserver_TabletServerAdminService_AddTableToTablet", "bytes","counter");
        table.insert("service_response_bytes_yb_tserver_TabletServerAdminService_AlterSchema", "bytes","counter");
        table.insert("service_response_bytes_yb_tserver_TabletServerAdminService_BackfillDone", "bytes","counter");
        table.insert("service_response_bytes_yb_tserver_TabletServerAdminService_BackfillIndex", "bytes","counter");
        table.insert("service_response_bytes_yb_tserver_TabletServerAdminService_CopartitionTable", "bytes","counter");
        table.insert("service_response_bytes_yb_tserver_TabletServerAdminService_CountIntents", "bytes","counter");
        table.insert("service_response_bytes_yb_tserver_TabletServerAdminService_CreateTablet", "bytes","counter");
        table.insert("service_response_bytes_yb_tserver_TabletServerAdminService_DeleteTablet", "bytes","counter");
        table.insert("service_response_bytes_yb_tserver_TabletServerAdminService_FlushTablets", "bytes","counter");
        table.insert("service_response_bytes_yb_tserver_TabletServerAdminService_GetSafeTime", "bytes","counter");
        table.insert("service_response_bytes_yb_tserver_TabletServerAdminService_PrepareDeleteTransactionTablet", "bytes","counter"); // 2.17
        table.insert("service_response_bytes_yb_tserver_TabletServerAdminService_PrepareDeleteTransactionTablet", "bytes","counter"); // 2.17
        table.insert("service_response_bytes_yb_tserver_TabletServerAdminService_RemoveTableFromTablet", "bytes","counter");
        table.insert("service_response_bytes_yb_tserver_TabletServerAdminService_SplitTablet", "bytes","counter");
        table.insert("service_response_bytes_yb_tserver_TabletServerAdminService_TabletSnapshotOp", "bytes","counter");
        table.insert("service_response_bytes_yb_tserver_TabletServerAdminService_TestRetry", "bytes","counter");
        table.insert("service_response_bytes_yb_tserver_TabletServerAdminService_UpdateConsensus", "bytes","counter");
        table.insert("service_response_bytes_yb_tserver_TabletServerAdminService_UpdateTransactionTablesVersion", "bytes","counter"); // 2.17
        table.insert("service_response_bytes_yb_tserver_TabletServerAdminService_UpgradeYsql", "bytes","counter");
        table.insert("service_response_bytes_yb_tserver_TabletServerBackupService_TabletSnapshotOp", "bytes","counter");
        table.insert("service_response_bytes_yb_tserver_TabletServerService_AbortTransaction", "bytes","counter");
        table.insert("service_response_bytes_yb_tserver_TabletServerService_Checksum", "bytes","counter");
        table.insert("service_response_bytes_yb_tserver_TabletServerService_GetLogLocation", "bytes","counter");
        table.insert("service_response_bytes_yb_tserver_TabletServerService_GetMasterAddresses", "bytes","counter");
        table.insert("service_response_bytes_yb_tserver_TabletServerService_GetSharedData", "bytes","counter");
        table.insert("service_response_bytes_yb_tserver_TabletServerService_GetSplitKey", "bytes","counter");
        table.insert("service_response_bytes_yb_tserver_TabletServerService_GetTabletStatus", "bytes","counter");
        table.insert("service_response_bytes_yb_tserver_TabletServerService_GetTransactionStatus", "bytes","counter");
        table.insert("service_response_bytes_yb_tserver_TabletServerService_GetTransactionStatusAtParticipant", "bytes","counter");
        table.insert("service_response_bytes_yb_tserver_TabletServerService_GetTserverCatalogVersionInfo", "bytes","counter");
        table.insert("service_response_bytes_yb_tserver_TabletServerService_ImportData", "bytes","counter");
        table.insert("service_response_bytes_yb_tserver_TabletServerService_IsTabletServerReady", "bytes","counter");
        table.insert("service_response_bytes_yb_tserver_TabletServerService_ListMasterServers","bytes","counter"); // 2.17.2
        table.insert("service_response_bytes_yb_tserver_TabletServerService_ListTablets", "bytes","counter");
        table.insert("service_response_bytes_yb_tserver_TabletServerService_ListTabletsForTabletServer", "bytes","counter");
        table.insert("service_response_bytes_yb_tserver_TabletServerService_NoOp", "bytes","counter");
        table.insert("service_response_bytes_yb_tserver_TabletServerService_ProbeTransactionDeadlock", "bytes","counter");
        table.insert("service_response_bytes_yb_tserver_TabletServerService_Publish", "bytes","counter");
        table.insert("service_response_bytes_yb_tserver_TabletServerService_Read", "bytes","counter");
        table.insert("service_response_bytes_yb_tserver_TabletServerService_TakeTransaction", "bytes","counter");
        table.insert("service_response_bytes_yb_tserver_TabletServerService_Truncate", "bytes","counter");
        table.insert("service_response_bytes_yb_tserver_TabletServerService_UpdateTransaction", "bytes","counter");
        table.insert("service_response_bytes_yb_tserver_TabletServerService_UpdateTransactionStatusLocation", "bytes","counter");
        table.insert("service_response_bytes_yb_tserver_TabletServerService_UpdateTransactionWaitingForStatus", "bytes","counter");
        table.insert("service_response_bytes_yb_tserver_TabletServerService_VerifyTableRowRange", "bytes","counter");
        table.insert("service_response_bytes_yb_tserver_TabletServerService_Write", "bytes","counter");
        table.insert("snapshot_operations_inflight", "operations","gauge");
        table.insert("spinlock_contention_time", "microseconds","counter");
        table.insert("split_operations_inflight", "operations","gauge");
        table.insert("sys_catalog_peer_write_count", "entries","counter");
        table.insert("tablet_data_corruptions", "corruptions","counter");
        table.insert("tcmalloc_current_total_thread_cache_bytes", "bytes","gauge");
        table.insert("tcmalloc_max_total_thread_cache_bytes", "bytes","gauge");
        table.insert("tcmalloc_pageheap_free_bytes", "bytes","gauge");
        table.insert("tcmalloc_pageheap_unmapped_bytes", "bytes","gauge");
        table.insert("tcp_bytes_received", "bytes","counter");
        table.insert("tcp_bytes_sent", "bytes","counter");
        table.insert("threads_running", "threads","gauge");
        table.insert("threads_running_CQLServer_reactor", "threads","gauge");
        table.insert("threads_running_Master_reactor", "threads","gauge");
        table.insert("threads_running_TabletServer_reactor", "threads","gauge");
        table.insert("threads_running_acceptor", "threads","gauge");
        table.insert("threads_running_catalog_manager", "threads","gauge");
        table.insert("threads_running_heartbeater", "threads","gauge");
        table.insert("threads_running_iotp_CQLServer", "threads","gauge");
        table.insert("threads_running_iotp_Master", "threads","gauge");
        table.insert("threads_running_iotp_TabletServer", "threads","gauge");
        table.insert("threads_running_iotp_call_home", "threads","gauge");
        table.insert("threads_running_maintenance", "threads","gauge");
        table.insert("threads_running_pg_supervisor", "threads","gauge");
        table.insert("threads_running_remote_bootstrap", "threads","gauge");
        table.insert("threads_running_remote_maintenance", "threads","gauge");
        table.insert("threads_running_rocksdb:high", "threads","gauge");
        table.insert("threads_running_rpc_thread_pool", "threads","gauge");
        table.insert("threads_running_tablet_manager", "threads","gauge");
        table.insert("threads_running_tablet_split_manager", "threads","gauge");
        table.insert("threads_running_thread_pool", "threads","gauge");
        table.insert("threads_started", "threads","counter");
        table.insert("threads_started_CQLServer_reactor", "threads","counter");
        table.insert("threads_started_Master_reactor", "threads","counter");
        table.insert("threads_started_TabletServer_reactor", "threads","gauge");
        table.insert("threads_started_acceptor", "threads","counter");
        table.insert("threads_started_auto_flags_client_reactor", "threads","counter"); // 2.15.2.1
        table.insert("threads_started_catalog_manager", "threads","counter");
        table.insert("threads_started_heartbeater", "threads","counter");
        table.insert("threads_started_iotp_CQLServer", "threads","counter");
        table.insert("threads_started_iotp_Master", "threads","counter");
        table.insert("threads_started_iotp_TabletServer", "threads","counter");
        table.insert("threads_started_iotp_auto_flags_client", "threads","counter"); // 2.15.2.1
        table.insert("threads_started_iotp_call_home", "threads","counter");
        table.insert("threads_started_maintenance", "threads","counter");
        table.insert("threads_started_pg_supervisor", "threads","counter");
        table.insert("threads_started_remote_bootstrap", "threads","counter");
        table.insert("threads_started_remote_maintenance", "threads","counter");
        table.insert("threads_started_rocksdb:high", "threads","gauge");
        table.insert("threads_started_rpc_thread_pool", "threads","counter");
        table.insert("threads_started_tablet_manager", "threads","counter");
        table.insert("threads_started_tablet_split_manager", "threads","counter");
        table.insert("threads_started_thread_pool", "threads","counter");
        table.insert("transaction_conflicts", "transactions","counter");
        table.insert("transaction_load_attempts", "transactions","counter");
        table.insert("transaction_not_found", "transactions","counter");
        table.insert("transaction_pool_cache_hits", "hits","counter");
        table.insert("transaction_pool_cache_queries", "queries","counter");
        table.insert("transaction_pool_prepared", "transactions","gauge");
        table.insert("transaction_pool_preparing", "transactions","gauge");
        table.insert("transactions_running", "transactions","gauge");
        table.insert("truncate_operations_inflight", "operations","gauge");
        table.insert("ts_post_split_compaction_added", "requests","gauge"); // 2.17
        table.insert("ts_split_compaction_added", "requests","gauge");
        table.insert("ts_split_op_added", "operations","gauge");
        table.insert("ts_split_op_apply", "operations","gauge");
        table.insert("update_transaction_operations_inflight", "operations","gauge");
        table.insert("voluntary_context_switches", "context switches","counter");
        table.insert("write_operations_inflight", "operations","gauge");
        table.insert("yb_cqlserver_CQLServerService_ParsingErrors", "requests","counter");
        table
    }
    /// Insert a row into the HashMap
    fn insert(
        &mut self,
        name: &str,
        unit: &str,
        statistic_type: &str
    )
    {
        self.valuestatisticdetails.insert(name.to_string(),
                                          ValueStatisticDetails { unit: unit.to_string(), unit_suffix: Self::suffix_lookup_value(unit), stat_type: statistic_type.to_string() }
        );
    }
    /// This creates a small lookup table to translate the full statistic type to the display version, which is abbreviated.
    /// This also helps to document the known statistic types.
    fn suffix_lookup_value(unit: &str) -> String {
        let suffix = HashMap::from([
            ("?", "?"),
            ("blocks", "blocks"),
            ("bytes", "bytes"),
            ("calls", "calls"),
            ("connections", "conn"),
            ("context switches", "csws"),
            ("corruptions", "corrupt"),
            ("current consensus term", "terms"),
            ("deletes", "dels"),
            ("entries", "entries"),
            ("failures", "fails"),
            ("files", "files"),
            ("hits", "hits"),
            ("indicator", "y/n"),
            ("iterators", "iters"),
            ("keys", "keys"),
            ("messages", "msgs"),
            ("microseconds", "us"),
            ("milliseconds", "ms"),
            ("nanoseconds", "ns"),
            ("nr", "nr"),
            ("operations", "ops"),
            ("parsers", "parsers"),
            ("processors", "procs"),
            ("properties", "props"),
            ("queries", "qry"),
            ("reads", "reads"),
            ("rejections", "reject"),
            ("requests", "reqs"),
            ("rows", "rows"),
            ("rpcs", "rpcs"),
            ("seeks", "seeks"),
            ("syncs", "syncs"),
            ("tasks", "tasks"),
            ("threads", "threads"),
            ("transactions", "txns"),
            ("writes", "writes"),
        ]);
        match suffix.get(unit) {
            Some(x) => x.to_string(),
            None           => {
                info!("The suffix for {} does not exist, please add to suffix_lookup_value!", unit);
                "?".to_string()
            },
        }
    }
}
/// These are the unit tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lookup_value_statistic_existing_name() {
        let value_statistics = ValueStatistics::create();
        let lookup = value_statistics.lookup("all_operations_inflight");
        assert_eq!(lookup.unit, "operations");
        assert_eq!(lookup.unit_suffix, "ops");
        assert_eq!(lookup.stat_type, "gauge");
    }

    #[test]
    fn lookup_value_statistic_non_existing_name() {
        let value_statistics = ValueStatistics::create();
        let lookup = value_statistics.lookup("does not exist");
        assert_eq!(lookup.unit, "?");
        assert_eq!(lookup.unit_suffix, "?");
        assert_eq!(lookup.stat_type, "?");
    }

}
