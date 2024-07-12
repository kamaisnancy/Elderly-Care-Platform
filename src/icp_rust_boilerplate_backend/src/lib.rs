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

// Helper function to increment ID
fn increment_id() -> u64 {
    ID_COUNTER.with(|counter| {
        let current_value = *counter.borrow().get();
        counter
            .borrow_mut()
            .set(current_value + 1)
            .expect("Failed to increment ID counter");
        current_value + 1
    })
}

// Function to create a new user
#[ic_cdk::update]
fn create_user(payload: UserPayload) -> Result<User, String> {
    // Ensure name and contact are not empty
    if payload.name.is_empty() || payload.contact.is_empty() {
        return Err("Name and contact cannot be empty".to_string());
    }

    let id = increment_id();

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

// Function to retrieve a user by ID
#[ic_cdk::query]
fn get_user_by_id(user_id: u64) -> Result<User, Error> {
    USERS_STORAGE.with(|storage| match storage.borrow().get(&user_id) {
        Some(user) => Ok(user.clone()),
        None => Err(Error::NotFound {
            msg: "User not found.".to_string(),
        }),
    })
}

// Function to update a user
#[ic_cdk::update]
fn update_user(user_id: u64, payload: UserPayload) -> Result<User, String> {
    USERS_STORAGE.with(|storage| {
        let mut storage = storage.borrow_mut();
        if let Some(mut user) = storage.remove(&user_id) {
            user.name = payload.name;
            user.contact = payload.contact;
            user.user_type = payload.user_type;
            storage.insert(user_id, user.clone());
            Ok(user)
        } else {
            Err("User not found".to_string())
        }
    })
}

// Function to delete a user
#[ic_cdk::update]
fn delete_user(user_id: u64) -> Result<(), String> {
    USERS_STORAGE.with(|storage| {
        if storage.borrow_mut().remove(&user_id).is_some() {
            Ok(())
        } else {
            Err("User not found".to_string())
        }
    })
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

    let id = increment_id();

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

// Function to retrieve a health record by ID
#[ic_cdk::query]
fn get_health_record_by_id(record_id: u64) -> Result<HealthRecord, Error> {
    HEALTH_RECORDS_STORAGE.with(|storage| match storage.borrow().get(&record_id) {
        Some(record) => Ok(record.clone()),
        None => Err(Error::NotFound {
            msg: "Health record not found.".to_string(),
        }),
    })
}

// Function to update a health record
#[ic_cdk::update]
fn update_health_record(record_id: u64, payload: HealthRecordPayload) -> Result<HealthRecord, String> {
    HEALTH_RECORDS_STORAGE.with(|storage| {
        let mut storage = storage.borrow_mut();
        if let Some(mut record) = storage.remove(&record_id) {
            record.heart_rate = payload.heart_rate;
            record.blood_pressure = payload.blood_pressure;
            record.activity_level = payload.activity_level;
            record.status = payload.status;
            storage.insert(record_id, record.clone());
            Ok(record)
        } else {
            Err("Health record not found".to_string())
        }
    })
}

// Function to delete a health record
#[ic_cdk::update]
fn delete_health_record(record_id: u64) -> Result<(), String> {
    HEALTH_RECORDS_STORAGE.with(|storage| {
        if storage.borrow_mut().remove(&record_id).is_some() {
            Ok(())
        } else {
            Err("Health record not found".to_string())
        }
    })
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

    let id = increment_id();

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

// Function to retrieve a medication reminder by ID
#[ic_cdk::query]
fn get_medication_reminder_by_id(reminder_id: u64) -> Result<MedicationReminder, Error> {
    MEDICATION_REMINDERS_STORAGE.with(|storage| match storage.borrow().get(&reminder_id) {
        Some(reminder) => Ok(reminder.clone()),
        None => Err(Error::NotFound {
            msg: "Medication reminder not found.".to_string(),
        }),
    })
}

// Function to update a medication reminder
#[ic_cdk::update]
fn update_medication_reminder(reminder_id: u64, payload: MedicationReminderPayload) -> Result<MedicationReminder, String> {
    MEDICATION_REMINDERS_STORAGE.with(|storage| {
        let mut storage = storage.borrow_mut();
        if let Some(mut reminder) = storage.remove(&reminder_id) {
            reminder.medication_name = payload.medication_name;
            reminder.dosage = payload.dosage;
            reminder.schedule = payload.schedule;
            storage.insert(reminder_id, reminder.clone());
            Ok(reminder)
        } else {
            Err("Medication reminder not found".to_string())
        }
    })
}

// Function to delete a medication reminder
#[ic_cdk::update]
fn delete_medication_reminder(reminder_id: u64) -> Result<(), String> {
    MEDICATION_REMINDERS_STORAGE.with(|storage| {
        if storage.borrow_mut().remove(&reminder_id).is_some() {
            Ok(())
        } else {
            Err("Medication reminder not found".to_string())
        }
    })
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

    let id = increment_id();

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

// Function to retrieve a virtual consultation by ID
#[ic_cdk::query]
fn get_virtual_consultation_by_id(consultation_id: u64) -> Result<VirtualConsultation, Error> {
    VIRTUAL_CONSULTATIONS_STORAGE.with(|storage| match storage.borrow().get(&consultation_id) {
        Some(consultation) => Ok(consultation.clone()),
        None => Err(Error::NotFound {
            msg: "Virtual consultation not found.".to_string(),
        }),
    })
}

// Function to update a virtual consultation
#[ic_cdk::update]
fn update_virtual_consultation(consultation_id: u64, payload: VirtualConsultationPayload) -> Result<VirtualConsultation, String> {
    VIRTUAL_CONSULTATIONS_STORAGE.with(|storage| {
        let mut storage = storage.borrow_mut();
        if let Some(mut consultation) = storage.remove(&consultation_id) {
            consultation.user_id = payload.user_id;
            consultation.provider_id = payload.provider_id;
            consultation.scheduled_at = payload.scheduled_at;
            consultation.status = payload.status;
            storage.insert(consultation_id, consultation.clone());
            Ok(consultation)
        } else {
            Err("Virtual consultation not found".to_string())
        }
    })
}

// Function to delete a virtual consultation
#[ic_cdk::update]
fn delete_virtual_consultation(consultation_id: u64) -> Result<(), String> {
    VIRTUAL_CONSULTATIONS_STORAGE.with(|storage| {
        if storage.borrow_mut().remove(&consultation_id).is_some() {
            Ok(())
        } else {
            Err("Virtual consultation not found".to_string())
        }
    })
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
