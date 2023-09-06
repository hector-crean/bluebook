

async : 


Widget vs. View: 

View: high level, declarative description of what the app wants to present
(simple value type) -> generates a widget tree
(builder for a widget?)

Widget: Interacts with the layout protocol, draws its appearance etc?




- rerun.io opensource
- fiberplane opensource



Conflict-Free Replicated Data Types (CRDTs) are data structures that allow multiple replicas to be updated concurrently and independently, without the need for consensus or a central coordinator. CRDTs can be used to represent Rust strings, providing strong eventual consistency guarantees.

There are several ways to represent strings using CRDTs. Some popular methods include:

Sequence CRDTs (e.g., RGA, LSEQ, and Logoot):
These CRDTs represent a string as a sequence of elements (characters), where each element has a unique identifier. The unique identifiers allow concurrent insertions and deletions to be merged without conflicts.

RGA (Replicated Growable Array): RGA uses a timestamped unique identifier for each character. It maintains a partial order of the characters based on their identifiers, allowing insertions and deletions to be merged without conflicts.

LSEQ (Logoot Split): LSEQ uses a hierarchical identifier structure that adapts to the density of elements in the sequence. This approach reduces the overhead of identifiers compared to other sequence CRDTs, making it more efficient in terms of space and time complexity.

Logoot: Logoot uses a fixed-size identifier for each character, which is constructed based on the position of the character in the sequence. The fixed-size identifiers help maintain a total order of the characters, ensuring that concurrent operations can be merged without conflicts.

State-based CRDTs (e.g., CmRDT, CvRDT):
State-based CRDTs use internal state to represent the string and track updates. There are two types of state-based CRDTs:

CmRDT (Commutative Replicated Data Type): CmRDTs rely on commutative update operations to achieve strong eventual consistency. For strings, a CmRDT would typically represent the string as a set of insertions and deletions, where each update operation is commutative and idempotent.

CvRDT (Convergent Replicated Data Type): CvRDTs use a merge function to combine the states of different replicas into a consistent state. For strings, a CvRDT would represent the string as a set or lattice structure, where the merge function combines the states of different replicas to achieve strong eventual consistency.