// solver lib

use std::{
    collections::{HashMap, HashSet},
    io::Error,
};

// board is the root obj that holds all entities
pub struct Board {
    // id -> obj
    pub resources: HashMap<String, Resource>,
    pub entities: HashMap<String, Entity>,
    // various relations
    pub id_relations: HashMap<String, IDRelation>,
    pub property_relations: HashMap<String, PropertyRelation>,
    pub id_property_relations: HashMap<String, IDPropertyRelation>,
    // entity id -> resource id.
    pub assignment: HashMap<String, String>,
}

impl Board {
    pub fn new() -> Board {
        return Board {
            resources: HashMap::new(),
            entities: HashMap::new(),
            id_relations: HashMap::new(),
            property_relations: HashMap::new(),
            id_property_relations: HashMap::new(),
            assignment: HashMap::new(),
        };
    }

    // success return true
    pub fn add_resource(&mut self, resource: Resource) -> bool {
        if self.resources.contains_key(&resource.id) {
            return false;
        }
        let op = self.resources.insert(resource.id.clone(), resource);
        assert!(op.is_none());
        return true;
    }

    pub fn add_entity(&mut self, resource_id: String, entity: Entity) -> bool {
        if !self.resources.contains_key(&resource_id) {
            return false; // resource not found.
        }
        let entity_id = entity.id.clone();
        if self.assignment.contains_key(&entity_id) {
            return false; // entity already assigned
        }
        if self.entities.contains_key(&entity_id) {
            return false; // entity already exist
        }
        let op = self.entities.insert(entity_id.clone(), entity);
        assert!(op.is_none());

        self.assignment.insert(entity_id, resource_id);

        return true;
    }

    // entities must be added before relations about them

    pub fn add_id_relation(&mut self, relation: IDRelation) -> Result<(), Error> {
        if self.id_relations.contains_key(&relation.id) {
            return Err(Error::new(
                std::io::ErrorKind::AlreadyExists,
                "id already exist",
            ));
        }
        if relation.kind == IDRelationKind::EEAffinity
            || relation.kind == IDRelationKind::EEAntiAffinity
        {
            // validate ids exist as entities
            if !self.entities.contains_key(&relation.id1)
                || !self.entities.contains_key(&relation.id2)
            {
                return Err(Error::new(
                    std::io::ErrorKind::NotFound,
                    "entity does not exist",
                ));
            }
        }
        if relation.kind == IDRelationKind::ERAffinity
            || relation.kind == IDRelationKind::ERAntiAffinity
        {
            // validate ids exist as entities
            if !self.entities.contains_key(&relation.id1) {
                return Err(Error::new(
                    std::io::ErrorKind::NotFound,
                    "entity does not exist",
                ));
            }
            if !self.resources.contains_key(&relation.id2) {
                return Err(Error::new(
                    std::io::ErrorKind::NotFound,
                    "resource does not exist",
                ));
            }
        }
        let op = self.id_relations.insert(relation.id.clone(), relation);
        assert!(op.is_none());
        Ok(())
    }

    pub fn add_property_relation(&mut self, relation: PropertyRelation) -> Result<(), Error> {
        if self.property_relations.contains_key(&relation.id) {
            return Err(Error::new(
                std::io::ErrorKind::AlreadyExists,
                "id already exist",
            ));
        }

        // no further validation done for property values.
        let op = self
            .property_relations
            .insert(relation.id.clone(), relation);
        assert!(op.is_none());
        Ok(())
    }

    pub fn add_id_property_relation(&mut self, relation: IDPropertyRelation) -> Result<(), Error> {
        if !self.entities.contains_key(&relation.entity_id) {
            return Err(Error::new(
                std::io::ErrorKind::NotFound,
                "entity does not exist",
            ));
        }
        let op = self
            .id_property_relations
            .insert(relation.id.clone(), relation);
        assert!(op.is_none());
        Ok(())
    }

    // TODO: change signature
    // currently it returns the ids of the property relation
    pub fn check_violation(&self) -> HashSet<String> {
        let mut property_violation: HashSet<String> = HashSet::new();

        // property check
        for (_, relation) in &self.property_relations {
            // check entity property matches resource property
            let ref ep = relation.entity_property;
            let ref rp = relation.resource_property;

            for (_, e) in &self.entities {
                if !e.properties.contains(ep) {
                    continue;
                }
                // find resource e is assigned to
                let assiged_r_id = self.assignment.get(&e.id).expect("assignment not found");
                let r = self.resources.get(assiged_r_id).expect("resouce not found");
                if !r.properties.contains(rp) {
                    property_violation.insert(relation.id.clone());
                }
            }
        }
        return property_violation;
    }
}

// stuff to be added to the board
// TODO: impl add Pending to Board.
pub struct Pending {
    // entities to be placed
    pub entities: HashMap<String, Entity>,
    // various relations
    pub id_relations: HashMap<String, IDRelation>,
    pub property_relations: HashMap<String, PropertyRelation>,
    pub id_property_relations: HashMap<String, IDPropertyRelation>,
}

// resource is the object that entities can bond to.
pub struct Resource {
    pub id: String,
    pub properties: HashSet<String>,
    pub capacities: HashMap<String, i64>,
}

impl Resource {
    pub fn new(id: String) -> Resource {
        return Resource {
            id: id,
            properties: HashSet::new(),
            capacities: HashMap::new(),
        };
    }

    pub fn add_property(&mut self, p: String) {
        let ok = self.properties.insert(p);
        assert!(ok);
    }
}

// entity can be bonded to one resource.
// different entities can bond to the same resource as long as capacity permits.
pub struct Entity {
    pub id: String,
    pub properties: HashSet<String>,
    pub metrics: HashMap<String, i64>,
    pub move_cost: i64, // move_cost low will be moved fisrt.
}

impl Entity {
    pub fn new(id: String) -> Entity {
        return Entity {
            id: id,
            properties: HashSet::new(),
            metrics: HashMap::new(),
            move_cost: 0,
        };
    }

    pub fn add_property(&mut self, p: String) {
        let ok = self.properties.insert(p);
        assert!(ok);
    }
}

#[derive(PartialEq)]
pub enum IDRelationKind {
    EEAffinity,
    EEAntiAffinity,
    ERAffinity,
    ERAntiAffinity,
}

// relation specifies relation between entities or entity-resources
// based on id.
// TODO: id relation can be replaced with property relation with unique properties,
// but property relation needs to support EE.
pub struct IDRelation {
    pub id: String,
    pub kind: IDRelationKind,
    // for EE relation, id1 and id2 are entity ids respectively
    // for ER relation, id1 is entity id, id2 is resource id.
    pub id1: String,
    pub id2: String,
}

#[derive(PartialEq)]
pub enum PropertyRelationKind {
    Affinity,
    AntiAffinity,
}

// property relation specifies entity's property in relation to resource's property
// TODO: property relation can be implemented as capacity consumption of unit 1,
// with resource with inifinit capacity, otherwise 0 capacity?
// For aniti-affinity, pick the resource with 0 capacity?
// For EE relation to be supported, select metrics greater than 0 or present?
pub struct PropertyRelation {
    pub id: String,
    pub kind: PropertyRelationKind,
    pub entity_property: String,
    pub resource_property: String,
}

// relation between entity and resource's property
// TODO: id property relation can be replaced by
// a property relation with entity with unique property.
pub struct IDPropertyRelation {
    pub id: String,
    pub entity_id: String,
    pub kind: PropertyRelationKind,
    pub resource_property: String,
}

#[cfg(test)]
mod tests {
    use crate::solver::PropertyRelation;

    use super::{Board, Entity, Resource};

    #[test]
    fn fabricclient_test() {
        assert!(true);
    }

    #[test]
    fn board_test() {
        let mut r1 = Resource::new(String::from("node1"));
        r1.add_property(String::from("red"));
        let mut r2 = Resource::new(String::from("node2"));
        r2.add_property(String::from("blue"));

        let mut e1 = Entity::new(String::from("app1"));
        e1.add_property(String::from("red"));

        let mut b = Board::new();
        let ok = b.add_resource(r1);
        assert!(ok);
        let ok = b.add_resource(r2);
        assert!(ok);
        // violation. blue node has red app
        let ok = b.add_entity(String::from("node2"), e1);
        assert!(ok);

        let rel1 = PropertyRelation {
            id: String::from("color"),
            kind: crate::solver::PropertyRelationKind::Affinity,
            entity_property: String::from("red"),
            resource_property: String::from("red"),
        };
        b.add_property_relation(rel1).expect("ok");

        let property_violation = b.check_violation();
        assert_eq!(property_violation.len(), 1);
        assert!(property_violation.contains(&String::from("color")));
    }
}
