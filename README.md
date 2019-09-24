# transfr
Transfr is a library for transferring files quickly between a client and server. Transfer is coordinated over a TCP connection and data is transferred via several UDP connections concurrently. Transferring data over UDP removes the overhead and throughput issues associated with TCP while maintaining some amount of transfer guarantees by coordinating over TCP.

## To Do Items
- [ ] Document protocol
- [ ] Server Implementation
- [ ] Client Implementation
- [ ] Testing

## transfr protocol
1. Handshake
    1. Client Identification Announcement
    1. Server Nonce Announcement
    1. Client Cnonce Announcement
    1. Server Acceptance Accouncement
1. Transfer Setup
    1. Client File Information Announcement
    1. Server Transfer Configuration Announcement 
1. Transfer 
    1. Control (TCP)
        1. Client Transfer Notifications
        1. Server Transfer Acknowledgements
    1. Data (UDP) 
