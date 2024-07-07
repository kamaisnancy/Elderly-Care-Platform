#[macro_use]
extern crate serde;
use candid::{Decode, Encode};
use ic_cdk::api::time;
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{BoundedStorable, Cell, DefaultMemoryImpl, StableBTreeMap, Storable};
use std::{borrow::Cow, cell::RefCell};

type Memory = VirtualMemory<DefaultMemoryImpl>;
type IdCell = Cell<u64, Memory>;

// UserType enum
#[derive(
    candid::CandidType, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash, Default, Debug,
)]
enum UserType {
    #[default]
    Elderly,
    Caregiver,
    HealthcareProvider,
}

// HealthStatus enum
#[derive(
    candid::CandidType, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash, Default, Debug,
)]
enum HealthStatus {
    #[default]
    Stable,
    Critical,
}

// User struct
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct User {
    id: u64,
    name: String,
    contact: String,
    user_type: UserType,
    created_at: u64,
}

// HealthRecord struct
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct HealthRecord {
    id: u64,
    user_id: u64,
    heart_rate: u8,
    blood_pressure: String,
    activity_level: String,
    status: HealthStatus,
    recorded_at: u64,
}

// MedicationReminder struct
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct MedicationReminder {
    id: u64,
    user_id: u64,
    medication_name: String,
    dosage: String,
    schedule: String,
    created_at: u64,
}

// VirtualConsultation struct
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct VirtualConsultation {
    id: u64,
    user_id: u64,
    provider_id: u64,
    scheduled_at: u64,
    status: String,
    created_at: u64,
}

impl Storable for User {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for User {
    const MAX_SIZE: u32 = 512;
    const IS_FIXED_SIZE: bool = false;
}

impl Storable for HealthRecord {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for HealthRecord {
    const MAX_SIZE: u32 = 512;
    const IS_FIXED_SIZE: bool = false;
}

impl Storable for MedicationReminder {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for MedicationReminder {
    const MAX_SIZE: u32 = 512;
    const IS_FIXED_SIZE: bool = false;
}

impl Storable for VirtualConsultation {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for VirtualConsultation {
    const MAX_SIZE: u32 = 512;
    const IS_FIXED_SIZE: bool = false;
}

thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );

    static ID_COUNTER: RefCell<IdCell> = RefCell::new(
        IdCell::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))), 0)
            .expect("Cannot create a counter")
    );

    static USERS_STORAGE: RefCell<StableBTreeMap<u64, User, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1)))
    ));

    static HEALTH_RECORDS_STORAGE: RefCell<StableBTreeMap<u64, HealthRecord, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(2)))
    ));

    static MEDICATION_REMINDERS_STORAGE: RefCell<StableBTreeMap<u64, MedicationReminder, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(3)))
    ));

    static VIRTUAL_CONSULTATIONS_STORAGE: RefCell<StableBTreeMap<u64, VirtualConsultation, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(4)))
    ));
}

// User Payload
#[derive(candid::CandidType, Deserialize, Serialize)]
struct UserPayload {
    name: String,
    contact: String,
    user_type: UserType,
}

// HealthRecord Payload
#[derive(candid::CandidType, Deserialize, Serialize)]
struct HealthRecordPayload {
    user_id: u64,
    heart_rate: u8,
    blood_pressure: String,
    activity_level: String,
    status: HealthStatus,
}

// MedicationReminder Payload
#[derive(candid::CandidType, Deserialize, Serialize)]
struct MedicationReminderPayload {
    user_id: u64,
    medication_name: String,
    dosage: String,
    schedule: String,
}

// VirtualConsultation Payload
#[derive(candid::CandidType, Deserialize, Serialize)]
struct VirtualConsultationPayload {
    user_id: u64,
    provider_id: u64,
    scheduled_at: u64,
    status: String,
}

// Function to create a new user
#[ic_cdk::update]
fn create_user(payload: UserPayload) -> Result<User, String> {
    // Ensure name and contact are not empty
    if payload.name.is_empty() || payload.contact.is_empty() {
        return Err("Name and contact cannot be empty".to_string());
    }

    let id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("Cannot increment ID counter");

    let user = User {
        id,
        name: payload.name,
        contact: payload.contact,
        user_type: payload.user_type,
        created_at: time(),
    };

    USERS_STORAGE.with(|storage| storage.borrow_mut().insert(id, user.clone()));
    Ok(user)
}

// Function to retrieve all users
#[ic_cdk::query]
fn get_all_users() -> Result<Vec<User>, String> {
    USERS_STORAGE.with(|storage| {
        let stable_btree_map = &*storage.borrow();
        let records: Vec<User> = stable_btree_map
            .iter()
            .map(|(_, record)| record.clone())
            .collect();
        if records.is_empty() {
            Err("No users found.".to_string())
        } else {
            Ok(records)
        }
    })
}

// Function to create a new health record
#[ic_cdk::update]
fn create_health_record(payload: HealthRecordPayload) -> Result<HealthRecord, String> {
    // Ensure all fields are provided
    if payload.blood_pressure.is_empty() || payload.activity_level.is_empty() {
        return Err("All fields must be provided.".to_string());
    }

    // Ensure user ID exists
    let user_exists = USERS_STORAGE.with(|storage| storage.borrow().contains_key(&payload.user_id));
    if !user_exists {
        return Err("User ID does not exist.".to_string());
    }

    let id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("Cannot increment ID counter");

    let health_record = HealthRecord {
        id,
        user_id: payload.user_id,
        heart_rate: payload.heart_rate,
        blood_pressure: payload.blood_pressure,
        activity_level: payload.activity_level,
        status: payload.status,
        recorded_at: time(),
    };

    HEALTH_RECORDS_STORAGE.with(|storage| storage.borrow_mut().insert(id, health_record.clone()));
    Ok(health_record)
}

// Function to retrieve all health records
#[ic_cdk::query]
fn get_all_health_records() -> Result<Vec<HealthRecord>, String> {
    HEALTH_RECORDS_STORAGE.with(|storage| {
        let stable_btree_map = &*storage.borrow();
        let records: Vec<HealthRecord> = stable_btree_map
            .iter()
            .map(|(_, record)| record.clone())
            .collect();
        if records.is_empty() {
            Err("No health records found.".to_string())
        } else {
            Ok(records)
        }
    })
}

// Function to create a new medication reminder
#[ic_cdk::update]
fn create_medication_reminder(
    payload: MedicationReminderPayload,
) -> Result<MedicationReminder, String> {
    // Ensure all fields are provided
    if payload.medication_name.is_empty()
        || payload.dosage.is_empty()
        || payload.schedule.is_empty()
    {
        return Err("All fields must be provided.".to_string());
    }
    
    // Ensure user ID exists
    let user_exists = USERS_STORAGE.with(|storage| storage.borrow().contains_key(&payload.user_id));
    if !user_exists {
        return Err("User ID does not exist.".to_string());
    }

    let id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("Cannot increment ID counter");

    let medication_reminder = MedicationReminder {
        id,
        user_id: payload.user_id,
        medication_name: payload.medication_name,
        dosage: payload.dosage,
        schedule: payload.schedule,
        created_at: time(),
    };

    MEDICATION_REMINDERS_STORAGE
        .with(|storage| storage.borrow_mut().insert(id, medication_reminder.clone()));
    Ok(medication_reminder)
}

// Function to retrieve all medication reminders
#[ic_cdk::query]
fn get_all_medication_reminders() -> Result<Vec<MedicationReminder>, String> {
    MEDICATION_REMINDERS_STORAGE.with(|storage| {
        let stable_btree_map = &*storage.borrow();
        let records: Vec<MedicationReminder> = stable_btree_map
            .iter()
            .map(|(_, record)| record.clone())
            .collect();
        if records.is_empty() {
            Err("No medication reminders found.".to_string())
        } else {
            Ok(records)
        }
    })
}

// Function to create a new virtual consultation
#[ic_cdk::update]
fn create_virtual_consultation(
    payload: VirtualConsultationPayload,
) -> Result<VirtualConsultation, String> {
    // Ensure all fields are provided
    if payload.user_id == 0 || payload.provider_id == 0 || payload.status.is_empty() {
        return Err("All fields must be provided.".to_string());
    }
    
    // Ensure user ID and provider ID exist
    let user_exists = USERS_STORAGE.with(|storage| storage.borrow().contains_key(&payload.user_id));
    if !user_exists {
        return Err("User ID does not exist.".to_string());
    }

    let provider_exists =
        USERS_STORAGE.with(|storage| storage.borrow().contains_key(&payload.provider_id));
    if !provider_exists {
        return Err("Provider ID does not exist.".to_string());
    }

    let id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("Cannot increment ID counter");

    let virtual_consultation = VirtualConsultation {
        id,
        user_id: payload.user_id,
        provider_id: payload.provider_id,
        scheduled_at: payload.scheduled_at,
        status: payload.status,
        created_at: time(),
    };

    VIRTUAL_CONSULTATIONS_STORAGE.with(|storage| {
        storage
            .borrow_mut()
            .insert(id, virtual_consultation.clone())
    });
    Ok(virtual_consultation)
}

// Function to retrieve all virtual consultations
#[ic_cdk::query]
fn get_all_virtual_consultations() -> Result<Vec<VirtualConsultation>, String> {
    VIRTUAL_CONSULTATIONS_STORAGE.with(|storage| {
        let stable_btree_map = &*storage.borrow();
        let records: Vec<VirtualConsultation> = stable_btree_map
            .iter()
            .map(|(_, record)| record.clone())
            .collect();
        if records.is_empty() {
            Err("No virtual consultations found.".to_string())
        } else {
            Ok(records)
        }
    })
}

// Error types
#[derive(candid::CandidType, Deserialize, Serialize)]
enum Error {
    NotFound { msg: String },
    UnAuthorized { msg: String },
}

// need this to generate candid
ic_cdk::export_candid!();
