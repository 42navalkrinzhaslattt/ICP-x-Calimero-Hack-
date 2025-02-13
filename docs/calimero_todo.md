# Calimero Exploration Todo List

## 1. Command Line Tools Setup and Basic Operations
- [x] Install and verify `merod`
- [x] Install and verify `meroctl`
- [x] Understand the difference between the two tools
- [x] Explore help commands and available options
- [ ] Debug current `meroctl` configuration loading issue

## 2. Node Management (`merod`)
- [x] Initialize a node with default settings
- [x] Initialize a node with custom ports
- [x] Run a node
- [x] Monitor node logs
- [ ] Configure node settings
- [ ] Understand node configuration file structure
- [ ] Try running multiple nodes on the same machine

## 3. Node Interaction (`meroctl`)
- [ ] Connect to a running node
- [ ] Check node status
- [ ] View node logs
- [ ] Explore interactive shell mode
- [ ] Test remote access capabilities
- [x] Understand JSON-RPC API endpoints

## 4. Identity Management
- [ ] Generate new identities
- [ ] List existing identities
- [ ] Understand the relationship between node identity and peer identity
- [ ] Explore identity permissions and capabilities
- [ ] Test identity management across multiple nodes

## 5. Application Management
- [ ] Install a sample application
- [ ] List installed applications
- [ ] Get application details
- [ ] Understand application deployment process
- [ ] Test application updates
- [ ] Explore application isolation

## 6. Context Operations
- [ ] Create a new context
- [ ] List existing contexts
- [ ] Join an existing context
- [ ] Invite peers to a context
- [ ] Monitor context events
- [ ] Test context synchronization
- [ ] Delete a context

## 7. Peer Interactions
- [ ] Create peers in different contexts
- [ ] Test peer-to-peer communication
- [ ] Monitor peer connections
- [ ] Understand peer discovery mechanisms
- [ ] Test peer authentication

## 8. Advanced Operations
- [ ] Test resource limits
- [ ] Explore VM isolation
- [ ] Monitor network traffic between nodes
- [ ] Test failover scenarios
- [ ] Explore backup and recovery options

## 9. Integration with ICP
- [ ] Understand ICP canister integration
- [ ] Test ICP-specific context creation
- [ ] Explore ICP identity management
- [ ] Test ICP state synchronization
- [ ] Monitor ICP-related events

## 10. Development and Testing
- [ ] Set up a local development environment
- [ ] Create test scenarios
- [ ] Debug common issues
- [ ] Monitor performance metrics
- [ ] Test security boundaries

## Known Issues to Investigate
1. `meroctl` Configuration Loading:
   - [ ] Verify config file format
   - [ ] Check file permissions
   - [ ] Test different configuration paths
   - [ ] Debug connection issues

2. Command Discrepancies:
   - [ ] Test all documented commands
   - [ ] Verify correct command syntax
   - [ ] Document working alternatives
   - [ ] Report inconsistencies

## Documentation Improvements
- [ ] Document successful command patterns
- [ ] Create troubleshooting guides
- [ ] Write example workflows
- [ ] Update command references
- [ ] Add integration guides 