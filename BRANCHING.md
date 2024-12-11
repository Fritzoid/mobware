# Branching

We use a kind og Git Flow branching strategy. 

We have 2 branches:

- main - This is the latest progress branch. We will branch from here when doing work and merge pull requests in here.
- stable - This is the branch with releases and normally it is the branch that reflects our production system.

Otherwise we have 3 kinds of branches. Feature branches, bug branches and hotfix branches. Called feature/{issue-number}-*, 
bug/{issue-number}-* and hotfix/{issue-number}-* respectively.   

Feature branches are created from main only and merge back to main. 
Bug branches are created from main only and merge back to main. 
Hotfix branches are created from stable only and merge back to stable. 

