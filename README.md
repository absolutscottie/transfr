# transfr
Transfr is a family of applications written with Rust to transfer files quickly from oneplace to another using UDP for transfer and TCP for coordination.


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
