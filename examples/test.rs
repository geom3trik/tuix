


use tuix::{EntityManager, Entity, Hierarchy2};


fn main() {
    let mut entity_manager = EntityManager::new();
    let mut hierarchy = Hierarchy2::new();

    println!("ONE");
    let entity1 = entity_manager.create_entity().unwrap();
    hierarchy.insert(entity1, Entity::root());

    println!("TWO");
    let entity2 = entity_manager.create_entity().unwrap();
    hierarchy.insert(entity2, Entity::root());

    // println!("THREE");
    // let entity3 = entity_manager.create_entity().unwrap();
    // hierarchy.insert(entity3, entity1);

    // println!("FOUR");
    // let entity4 = entity_manager.create_entity().unwrap();
    // hierarchy.insert(entity4, entity3);

    // let entity5 = entity_manager.create_entity().unwrap();
    // hierarchy.insert(entity5, entity1);

    // let entity6 = entity_manager.create_entity().unwrap();
    // hierarchy.insert(entity6, entity2);


    println!("{:?}", hierarchy.meta);
    //println!("{:?}", hierarchy.entities);

    for item in hierarchy.entities.iter() {
        print!("({} {})", item.entity, item.parent);
    }

    // hierarchy.remove(entity3);

    // println!("{:?}", hierarchy.indices);
    // println!("{:?}", hierarchy.entities);

    //println!("First Child: {:?}", hierarchy.get_first_child(entity1));
}