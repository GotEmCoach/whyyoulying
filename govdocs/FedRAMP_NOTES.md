# FedRAMP Applicability — whyyoulying

## Deployment Model

whyyoulying is a **local CLI tool**, not a cloud service. It runs on the investigator's workstation, reads local files, and exits.

| Characteristic | Value |
|---------------|-------|
| Deployment type | On-premise / local workstation |
| Cloud service? | No |
| SaaS? | No |
| Multi-tenant? | No |
| Internet required? | No |
| Persistent process? | No (runs and exits) |

## FedRAMP Authorization

**FedRAMP authorization is not applicable.** FedRAMP applies to cloud services offered to federal agencies. whyyoulying is:
- Not a cloud service
- Not offered as-a-service
- Not multi-tenant
- Not internet-connected during operation

## If Deployed as a Service (Future)

If whyyoulying were wrapped in a web service or API for shared use:
- FedRAMP Li-SaaS or Low baseline would apply
- Authorization boundary would include: the host, the API layer, and input/output data flows
- Data classification: CUI (Controlled Unclassified Information) likely, given DoD contract/employee data
- Required: ATO from sponsoring agency, 3PAO assessment, continuous monitoring

## Current Boundary

```
[User Workstation] → whyyoulying binary → [stdout / local file]
                   ↑
              [local JSON files]
```

No network boundary. No data leaves the workstation.
