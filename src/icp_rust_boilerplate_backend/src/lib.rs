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

// MealType enum
#[derive(
    candid::CandidType, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash, Default, Debug,
)]
enum MealType {
    #[default]
    Breakfast,
    Lunch,
    Dinner,
}

// ExerciseType enum
#[derive(
    candid::CandidType, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash, Default, Debug,
)]
enum ExerciseType {
    #[default]
    Cardio,
    Strength,
    Flexibility,
}

// Intensity enum
#[derive(
    candid::CandidType, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash, Default, Debug,
)]
enum Intensity {
    #[default]
    Low,
    Medium,
    High,
}

// Mood enum
#[derive(
    candid::CandidType, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash, Default, Debug,
)]
enum Mood {
    #[default]
    Happy,
    Sad,
    Anxious,
}

// StressLevel enum
#[derive(
    candid::CandidType, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash, Default, Debug,
)]
enum StressLevel {
    #[default]
    Low,
    Medium,
    High,
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

// DietRecord struct
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct DietRecord {
    id: u64,
    user_id: u64,
    meal_type: MealType,
    food_items: String, // Comma-separated list of food items
    calories: u32,
    recorded_at: u64,
}

// ExerciseRecord struct
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct ExerciseRecommendation {
    id: u64,
    user_id: u64,
    exercise_type: ExerciseType,
    duration: u32, // in minutes
    intensity: Intensity,
    recommended_at: u64,
}

// MentalHealthRecord struct
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct MentalHealthRecord {
    id: u64,
    user_id: u64,
    mood: Mood,
    stress_level: StressLevel,
    notes: String, // Any additional notes
    recorded_at: u64,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct FitnessChallenge {
    id: u64,
    name: String,
    description: String,
    start_date: u64,
    end_date: u64,
    created_at: u64,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct FitnessChallengeParticipant {
    id: u64,
    challenge_id: u64,
    user_id: u64,
    progress: u32, // e.g., steps walked, distance covered, etc.
    updated_at: u64,
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

impl Storable for DietRecord {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for DietRecord {
    const MAX_SIZE: u32 = 512;
    const IS_FIXED_SIZE: bool = false;
}

impl Storable for ExerciseRecommendation {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for ExerciseRecommendation {
    const MAX_SIZE: u32 = 512;
    const IS_FIXED_SIZE: bool = false;
}

impl Storable for MentalHealthRecord {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for MentalHealthRecord {
    const MAX_SIZE: u32 = 512;
    const IS_FIXED_SIZE: bool = false;
}

impl Storable for FitnessChallenge {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for FitnessChallenge {
    const MAX_SIZE: u32 = 512;
    const IS_FIXED_SIZE: bool = false;
}

impl Storable for FitnessChallengeParticipant {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for FitnessChallengeParticipant {
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

    static DIET_RECORDS_STORAGE: RefCell<StableBTreeMap<u64, DietRecord, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(5)))
    ));

    static EXERCISE_RECOMMENDATIONS_STORAGE: RefCell<StableBTreeMap<u64, ExerciseRecommendation, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(6)))
    ));

    static MENTAL_HEALTH_RECORDS_STORAGE: RefCell<StableBTreeMap<u64, MentalHealthRecord, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(7)))
    ));

    static FITNESS_CHALLENGES_STORAGE: RefCell<StableBTreeMap<u64, FitnessChallenge, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(8)))
    ));

    static FITNESS_CHALLENGE_PARTICIPANTS_STORAGE: RefCell<StableBTreeMap<u64, FitnessChallengeParticipant, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(9)))
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

// DietRecord Payload
#[derive(candid::CandidType, Deserialize, Serialize)]
struct DietRecordPayload {
    user_id: u64,
    meal_type: MealType,
    food_items: String,
    calories: u32,
}

// ExerciseRecommendation Payload
#[derive(candid::CandidType, Deserialize, Serialize)]
struct ExerciseRecommendationPayload {
    user_id: u64,
    exercise_type: ExerciseType,
    duration: u32,
    intensity: Intensity,
}

// MentalHealthRecord Payload
#[derive(candid::CandidType, Deserialize, Serialize)]
struct MentalHealthRecordPayload {
    user_id: u64,
    mood: Mood,
    stress_level: StressLevel,
    notes: String,
}

// FitnessChallenge Payload
#[derive(candid::CandidType, Deserialize, Serialize)]
struct FitnessChallengePayload {
    name: String,
    description: String,
    start_date: u64,
    end_date: u64,
}

// FitnessChallengeParticipant Payload
#[derive(candid::CandidType, Deserialize, Serialize)]
struct FitnessChallengeParticipantPayload {
    challenge_id: u64,
    user_id: u64,
    progress: u32,
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
    if payload.status.is_empty() {
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

// Function to create a new diet record
#[ic_cdk::update]
fn create_diet_record(payload: DietRecordPayload) -> Result<DietRecord, String> {
    // Ensure all fields are provided
    if payload.food_items.is_empty() {
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

    let diet_record = DietRecord {
        id,
        user_id: payload.user_id,
        meal_type: payload.meal_type,
        food_items: payload.food_items,
        calories: payload.calories,
        recorded_at: time(),
    };

    DIET_RECORDS_STORAGE.with(|storage| storage.borrow_mut().insert(id, diet_record.clone()));
    Ok(diet_record)
}

// Function to retrieve diet records by user ID
#[ic_cdk::query]
fn get_diet_records_by_user_id(user_id: u64) -> Result<Vec<DietRecord>, String> {
    DIET_RECORDS_STORAGE.with(|storage| {
        let stable_btree_map = &*storage.borrow();
        let records: Vec<DietRecord> = stable_btree_map
            .iter()
            .filter(|(_, record)| record.user_id == user_id)
            .map(|(_, record)| record.clone())
            .collect();
        if records.is_empty() {
            Err("No diet records found.".to_string())
        } else {
            Ok(records)
        }
    })
}

// Function to retrieve all diet records
#[ic_cdk::query]
fn get_all_diet_records() -> Result<Vec<DietRecord>, String> {
    DIET_RECORDS_STORAGE.with(|storage| {
        let stable_btree_map = &*storage.borrow();
        let records: Vec<DietRecord> = stable_btree_map
            .iter()
            .map(|(_, record)| record.clone())
            .collect();
        if records.is_empty() {
            Err("No diet records found.".to_string())
        } else {
            Ok(records)
        }
    })
}

// Function to create a new exercise recommendation
#[ic_cdk::update]
fn create_exercise_recommendation(
    payload: ExerciseRecommendationPayload,
) -> Result<ExerciseRecommendation, String> {
    // Ensure all fields are provided
    if payload.duration == 0 {
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

    let recommendation = ExerciseRecommendation {
        id,
        user_id: payload.user_id,
        exercise_type: payload.exercise_type,
        duration: payload.duration,
        intensity: payload.intensity,
        recommended_at: time(),
    };

    EXERCISE_RECOMMENDATIONS_STORAGE
        .with(|storage| storage.borrow_mut().insert(id, recommendation.clone()));
    Ok(recommendation)
}

// Function to retrieve exercise recommendations by user ID
#[ic_cdk::query]
fn get_exercise_recommendations_by_user_id(
    user_id: u64,
) -> Result<Vec<ExerciseRecommendation>, String> {
    EXERCISE_RECOMMENDATIONS_STORAGE.with(|storage| {
        let stable_btree_map = &*storage.borrow();
        let records: Vec<ExerciseRecommendation> = stable_btree_map
            .iter()
            .filter(|(_, record)| record.user_id == user_id)
            .map(|(_, record)| record.clone())
            .collect();
        if records.is_empty() {
            Err("No exercise recommendations found.".to_string())
        } else {
            Ok(records)
        }
    })
}

// Function to retrieve all exercise recommendations
#[ic_cdk::query]
fn get_all_exercise_recommendations() -> Result<Vec<ExerciseRecommendation>, String> {
    EXERCISE_RECOMMENDATIONS_STORAGE.with(|storage| {
        let stable_btree_map = &*storage.borrow();
        let records: Vec<ExerciseRecommendation> = stable_btree_map
            .iter()
            .map(|(_, record)| record.clone())
            .collect();
        if records.is_empty() {
            Err("No exercise recommendations found.".to_string())
        } else {
            Ok(records)
        }
    })
}

// Function to create a new mental health record
#[ic_cdk::update]
fn create_mental_health_record(
    payload: MentalHealthRecordPayload,
) -> Result<MentalHealthRecord, String> {
    // Ensure all fields are provided
    if payload.notes.is_empty() {
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

    let mental_health_record = MentalHealthRecord {
        id,
        user_id: payload.user_id,
        mood: payload.mood,
        stress_level: payload.stress_level,
        notes: payload.notes,
        recorded_at: time(),
    };

    MENTAL_HEALTH_RECORDS_STORAGE.with(|storage| {
        storage
            .borrow_mut()
            .insert(id, mental_health_record.clone())
    });
    Ok(mental_health_record)
}

// Function to retrieve mental health records by user ID
#[ic_cdk::query]
fn get_mental_health_records_by_user_id(user_id: u64) -> Result<Vec<MentalHealthRecord>, String> {
    MENTAL_HEALTH_RECORDS_STORAGE.with(|storage| {
        let stable_btree_map = &*storage.borrow();
        let records: Vec<MentalHealthRecord> = stable_btree_map
            .iter()
            .filter(|(_, record)| record.user_id == user_id)
            .map(|(_, record)| record.clone())
            .collect();
        if records.is_empty() {
            Err("No mental health records found.".to_string())
        } else {
            Ok(records)
        }
    })
}

// Function to retrieve all mental health records
#[ic_cdk::query]
fn get_all_mental_health_records() -> Result<Vec<MentalHealthRecord>, String> {
    MENTAL_HEALTH_RECORDS_STORAGE.with(|storage| {
        let stable_btree_map = &*storage.borrow();
        let records: Vec<MentalHealthRecord> = stable_btree_map
            .iter()
            .map(|(_, record)| record.clone())
            .collect();
        if records.is_empty() {
            Err("No mental health records found.".to_string())
        } else {
            Ok(records)
        }
    })
}

// Function to create a new fitness challenge
#[ic_cdk::update]
fn create_fitness_challenge(payload: FitnessChallengePayload) -> Result<FitnessChallenge, String> {
    if payload.name.is_empty() || payload.description.is_empty() {
        return Err("Name and description cannot be empty".to_string());
    }

    let id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("Cannot increment ID counter");

    let challenge = FitnessChallenge {
        id,
        name: payload.name,
        description: payload.description,
        start_date: payload.start_date,
        end_date: payload.end_date,
        created_at: time(),
    };

    FITNESS_CHALLENGES_STORAGE.with(|storage| storage.borrow_mut().insert(id, challenge.clone()));
    Ok(challenge)
}

// Function to create a new fitness challenge participant
#[ic_cdk::update]
fn create_fitness_challenge_participant(
    payload: FitnessChallengeParticipantPayload,
) -> Result<FitnessChallengeParticipant, String> {
    let challenge_exists = FITNESS_CHALLENGES_STORAGE
        .with(|storage| storage.borrow().contains_key(&payload.challenge_id));
    if !challenge_exists {
        return Err("Challenge ID does not exist.".to_string());
    }

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

    let participant = FitnessChallengeParticipant {
        id,
        challenge_id: payload.challenge_id,
        user_id: payload.user_id,
        progress: payload.progress,
        updated_at: time(),
    };

    FITNESS_CHALLENGE_PARTICIPANTS_STORAGE
        .with(|storage| storage.borrow_mut().insert(id, participant.clone()));
    Ok(participant)
}

// Function to retrieve all fitness challenges
#[ic_cdk::query]
fn get_all_fitness_challenges() -> Result<Vec<FitnessChallenge>, String> {
    FITNESS_CHALLENGES_STORAGE.with(|storage| {
        let stable_btree_map = &*storage.borrow();
        let records: Vec<FitnessChallenge> = stable_btree_map
            .iter()
            .map(|(_, record)| record.clone())
            .collect();
        if records.is_empty() {
            Err("No fitness challenges found.".to_string())
        } else {
            Ok(records)
        }
    })
}

// Function to retrieve all fitness challenge participants
#[ic_cdk::query]
fn get_all_fitness_challenge_participants() -> Result<Vec<FitnessChallengeParticipant>, String> {
    FITNESS_CHALLENGE_PARTICIPANTS_STORAGE.with(|storage| {
        let stable_btree_map = &*storage.borrow();
        let records: Vec<FitnessChallengeParticipant> = stable_btree_map
            .iter()
            .map(|(_, record)| record.clone())
            .collect();
        if records.is_empty() {
            Err("No fitness challenge participants found.".to_string())
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
