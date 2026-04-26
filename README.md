
```
SystemActorSupervisor
         │
         ▼
    PipelineSupervisor
          │
          |
          |    
          ->  [0] consumer → processor → store (pipeline 0)
          ->  [1] consumer → processor → store (pipeline 1)
          ->  [2] consumer → processor → store (pipeline 2)

```