# ARCHITECTURE
// every app contains two layers
- view layer     
    - view
    - view model
- data layer 
    - repos
    - services

// VIEW_LAYER:
// one feature - one view & one view model
    // VIEW:
    // just UI with minimum  logic
    // - injections of view_model
    // - colbacks for buttons
    // - condition widget rendering
    // VIEW_MODEL:
    // - retriving data for view
    // - imutable view state
    // - view logic
