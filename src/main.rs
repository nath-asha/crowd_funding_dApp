mod project;
mod blockchain;

use project::Project;
use blockchain::Blockchain;

fn main() {
    // Simulated blockchain interaction
    let mut blockchain = Blockchain::new();
    
    // Create a crowdfunding project
    let mut project = Project::new("Project A".to_string(), 1000, 10);
    
    // Simulated user contributions
    project.contribute(1, 20, &mut blockchain);
    project.contribute(2, 30, &mut blockchain);
    
    // Check if the target is reached
    if project.check_target_reached() {
        project.disburse_funds(&mut blockchain);
    }
}
