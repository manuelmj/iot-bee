SystemActorSupervisor
    HashMap<pipeline_id, Addr<PipelineSupervisor>>   ← aquí vive el ID
         │
         ▼
    PipelineSupervisor  (sabe que es el pipeline X, pero internamente...)
         Vec<Arc<PipelineAbstractionController>>      ← solo réplicas, sin ID
              [0] consumer → processor → store
              [1] consumer → processor → store
              [2] consumer → processor → store