### Design Diagrams
This folder has the details of the intended 
structure of the software.

#### Sequence Diagrams
They illustrate the fashion in which the software
will be expected to operate.  
They are located in 'Sequence Diagrams'.

#### Class Diagrams
They provide the members of each class/interface.  
None have been made yet.

#### Object Diagrams
They explain how the classes interact with each other.  
They are located in 'Object Diagrams'.

### Design and Reasoning

This follows TEA model of GUI, with some elements of MVC.

Why? We often perform I/O from the same context, 
and so it makes sense to wrap them together.
Further, I/O sends and receives messages from the model, 
which readily maps to TEA.

Finally, most GUIs today follow TEA model, 
and so adopting it means less work to get started.

Faction details and other data is to be stored in external files
(`.ini` as of now). This allows them to be
modded/rebalanced without building anew.
(Present in src/config to avoid conflict with user settings.)

### Major Goals

0. Working display and input (done in part)
1. Working data storage (wip)
   1.1. Config files to load creature specs
   1.2. Savefiles using (likely) Serde
2. Working game logic
3. Faction implementation
4. Story implementation