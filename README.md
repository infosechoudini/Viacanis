![RUST](https://github.com/infosechoudini/Viacanis/workflows/build/badge.svg?branch=master) 

# Viacanis
Blue Team Threat Hunting and Monitoring Tool

# Purpose
To open source blue team tools and gather input from the community. 

# To Do 
- Make this readme better
  - Add install and running commands
  - ASCII Art
  - I guess code more for this project...

# Roadmap
  ## Version (0.1.0)
    - Telemetry for Agents
    - Agent interaction via Standalone Server
    - Communication between Server and Agent are encrypted
    - Add All Additional Windows Event Threat Behaviors
      - Maybe not all, but a majority
    - Reduce resource use
      - Add efficiencies in monitoring
  ## Version (0.2.0) 
    - Port PESieve to Rust (PESieve license included)
      - Make it async/concurrent
    - Attach to processes like a debugger?
      - Might be too resource intensive
  ## Version (0.3.0)
    - Add some form of reactions
      - Halt a Process?
      - Carve out the malicious code from a process?
  ## Version (0.4.0)
    - Add backwards compatibility
      - Windows 7
      - Windows XP

 
# Inspiration
The main inspiration of this tool is Bluespawn and thusly PE-SIEVE. I like their work and what their doing. I'm looking to create something for more resource constrained environments like ICS/OT endpoints. 

# Caveat
I do not recommend running this in Production. This tool is purely research at this point. Use at your own risk. 
