
/// Committee type 
/// 
/// contains information about the Committee
/// including their related works, member
struct CommonsCommittee {
    name: String,
    committee_type: String,
    works: Vec<Work>,
    members: Vec<CommitteeMember>
}

struct CommitteeMember {
    person: Mp,
    postision: String
}

struct Mp {
    title: String,
    name: String,
    party: String,
    constituency: String
}

struct Work {
    name: String
}

fn get_common_committees() -> Vec<CommonsCommittee> {
    vec![
        CommonsCommittee {
            name: "Administration Committee".to_string(),
            committee_type: "Select Committee".to_string(),
            works: Vec::new(),
            members: vec![
                CommitteeMember {
                    person: Mp { 
                        title: "Player".to_string(), 
                        name: "Breadddd 231".to_string(), 
                        party: "Liberal".to_string(), 
                        constituency: "North Carboxy Castle".to_string() 
                    },
                    postision: "Chair".to_string()
                },
                CommitteeMember {
                    person: Mp {
                        title: "Player".to_string(),
                        name: "FFFFFFF".to_string(),
                        party: "For Player".to_string(),
                        constituency: "Ur Mom".to_string()
                    },
                    postision: "Deputy Chair".to_string()
                }
            ]
        }
    ]
}

