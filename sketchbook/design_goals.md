From HURD:

Potential design principles
Here is an incomplete list of potential design principles for the ngHurd. It is taken from [2]. I left out some principles I think do not apply or are not in question. Feel free to add more.

Principles from the Multics Project
Economy of mechanism: Keep the design as simple as possible.
Fail-safe defaults: Base access decisions on permission rather than exclusion.
Least priviledge: Components should have no more authority than they require.
Least common mechanism: Minimize the amount of shared instances in the system.
Commonly accepted principles
Separation of policy and mechanism
Least astonishment (also known as principle of least surprise): The system�s behavior should match what is naively expected.
Complete accountability: All real resources held by an application must come from some accounted pool.
Safe restart: On restart, the system must either already have, or be able to rapidly establish, a consistent and secure execution state.
Reproducibility: Correct operations should produce identical results regardless of workload.
Principles specific to EROS
Credible policy: If a security policy cannot be implemented by correct application of the system�s protection mechanisms, do not claim to enforce it.
Explicit authority designation: Every operation that uses authority should explicitely designate the source of the authority it is using.
Relinquishable authority: If an application holds some authority, it should be able to voluntarily reduce this authority.