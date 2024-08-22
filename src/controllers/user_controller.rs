use crate::{
    common_struct::{handle_db_error, handle_invalid_id_error, ApiResponse},
    db,
    models::user_module::User,
};
use axum::{extract::Path, http::StatusCode, Json};
use mongodb::bson::{doc, oid::ObjectId, DateTime, Document};
use serde_json::{json, Value};

pub async fn add_user(Json(mut payload): Json<User>) -> (StatusCode, Json<Value>) {
    let db = match db::connect_db().await {
        Ok(db) => db,
        Err(error) => return handle_db_error(error).await,
    };

    let coll = db.collection::<User>("users");

    if payload.first_name.is_none()
        || payload.last_name.is_none()
        || payload.email.is_none()
        || payload.password.is_none()
    {
        println!("Missing fields in payload: {:?}", payload);
        return (
            StatusCode::BAD_REQUEST,
            Json(json!(ApiResponse {
                status: "error".to_string(),
                code: 400,
                message: "Missing fields".to_string(),
                data: None,
                errors: Some(format!("Missing fields: {:?}", payload)),
            })),
        );
    }

    payload.created_at = Some(DateTime::now());
    payload.updated_at = Some(DateTime::now());

    match coll.insert_one(payload).await {
        Ok(res) => {
            println!("User Added With ID: {}", res.inserted_id);
            (
                StatusCode::OK,
                Json(json!(ApiResponse {
                    status: "Success".to_string(),
                    code: 200,
                    message: "User added successfully".to_string(),
                    data: Some(format!("id:{}", res.inserted_id)),
                    errors: None,
                })),
            )
        }
        Err(error) => handle_db_error(error).await,
    }
}

pub async fn get_user(Path(params): Path<String>) -> (StatusCode, Json<Value>) {
    let db = match db::connect_db().await {
        Ok(db) => db,
        Err(error) => return handle_db_error(error).await,
    };

    let coll = db.collection::<Document>("users");

    let oid = match ObjectId::parse_str(&params) {
        Ok(oid) => oid,
        Err(_) => return handle_invalid_id_error::<String>(params).await,
    };

    match coll.find_one(doc! {"_id": oid}).await {
        Ok(Some(data)) => {
            println!("User Details: {:?}", data);
            (
                StatusCode::OK,
                Json(json!(ApiResponse {
                    status: "Success".to_string(),
                    code: 200,
                    message: "User retrieved successfully".to_string(),
                    data: Some(data),
                    errors: None,
                })),
            )
        }
        Ok(None) => {
            println!("User Not Found with ID: {}", params);
            (
                StatusCode::NOT_FOUND,
                Json(json!(ApiResponse {
                    status: "error".to_string(),
                    code: 404,
                    message: "User not found".to_string(),
                    data: None,
                    errors: Some(format!("User not found with ID: {}", params)),
                })),
            )
        }
        Err(error) => handle_db_error(error).await,
    }
}

pub async fn update_user(
    Path(params): Path<String>,
    Json(payload): Json<User>,
) -> (StatusCode, Json<Value>) {
    let db = match db::connect_db().await {
        Ok(db) => db,
        Err(error) => return handle_db_error(error).await,
    };

    let coll = db.collection::<User>("users");

    let oid = match ObjectId::parse_str(&params) {
        Ok(oid) => oid,
        Err(_) => return handle_invalid_id_error::<String>(params).await,
    };

    let mut update_doc = doc! {};

    if let Some(first_name) = &payload.first_name {
        update_doc.insert("firstName", first_name);
    }
    if let Some(last_name) = &payload.last_name {
        update_doc.insert("lastName", last_name);
    }
    if let Some(email) = &payload.email {
        update_doc.insert("email", email);
    }
    if let Some(password) = &payload.password {
        update_doc.insert("password", password);
    }

    if !update_doc.is_empty() {
        update_doc.insert("updatedAt", DateTime::now());
        match coll
            .update_one(doc! {"_id": oid}, doc! { "$set": update_doc })
            .await
        {
            Ok(res) => {
                println!(
                    "Matched {} document(s) and modified {} document(s)",
                    res.matched_count, res.modified_count
                );
                (
                    StatusCode::OK,
                    Json(json!(ApiResponse {
                        status: "Success".to_string(),
                        code: 200,
                        message: "User updated successfully".to_string(),
                        data: Some(format!(
                            "Matched {} document(s) and modified {} document(s)",
                            res.matched_count, res.modified_count
                        )),
                        errors: None,
                    })),
                )
            }
            Err(error) => handle_db_error(error).await,
        }
    } else {
        println!("No fields to update for user with ID: {}", params);
        (
            StatusCode::BAD_REQUEST,
            Json(json!(ApiResponse {
                status: "error".to_string(),
                code: 400,
                message: "No fields to update".to_string(),
                data: None,
                errors: Some(format!("No fields to update for user with ID: {}", params)),
            })),
        )
    }
}

pub async fn delete_user(Path(params): Path<String>) -> (StatusCode, Json<Value>) {
    let db = match db::connect_db().await {
        Ok(db) => db,
        Err(error) => return handle_db_error(error).await,
    };

    let coll = db.collection::<User>("users");

    let oid = match ObjectId::parse_str(&params) {
        Ok(oid) => oid,
        Err(_) => return handle_invalid_id_error::<String>(params).await,
    };

    match coll.delete_one(doc! {"_id": oid}).await {
        Ok(res) => {
            if res.deleted_count > 0 {
                println!("User Deleted with ID: {}", params);
                (
                    StatusCode::OK,
                    Json(json!(ApiResponse {
                        status: "Success".to_string(),
                        code: 200,
                        message: "User deleted successfully".to_string(),
                        data: Some(format!("Deleted {} user(s)", res.deleted_count)),
                        errors: None,
                    })),
                )
            } else {
                println!("User Not Found with ID: {}", params);
                (
                    StatusCode::NOT_FOUND,
                    Json(json!(ApiResponse {
                        status: "error".to_string(),
                        code: 404,
                        message: "User not found".to_string(),
                        data: None,
                        errors: Some(format!("User not found with ID: {}", params)),
                    })),
                )
            }
        }
        Err(error) => handle_db_error(error).await,
    }
}

// pub async fn add_user(Json(mut payload): Json<User>) -> (StatusCode, Json<Value>) {
//     let db = db::connect_db().await;
//     match db {
//         Ok(db) => {
//             let coll = db.collection::<User>("users");
//             println!(
//                 "{},{},{},{}",
//                 payload.first_name.is_none(),
//                 payload.last_name.is_none(),
//                 payload.email.is_none(),
//                 payload.password.is_none()
//             );
//             if !payload.first_name.is_none()
//                 && !payload.last_name.is_none()
//                 && !payload.email.is_none()
//                 && !payload.password.is_none()
//             {
//                 payload.created_at = Some(DateTime::now());
//                 payload.updated_at = Some(DateTime::now());
//                 let add_user = coll.insert_one(payload).await;
//                 match add_user {
//                     Ok(res) => {
//                         println!("User Added With ID:{}", res.inserted_id);
//                         let json_response = json!(ApiResponse {
//                             status: "Success".to_string(),
//                             code: 200,
//                             message: "User Added Successfully".to_string(),
//                             data: Some(format!("id:{}", res.inserted_id)),
//                             errors: None,
//                         });
//                         (StatusCode::OK, Json(json_response))
//                     }
//                     Err(error) => {
//                         println!("Error While Adding User:{}", error);
//                         let json_response = json!(ApiResponse {
//                             status: "error".to_string(),
//                             code: 500,
//                             message: "Internal server error".to_string(),
//                             data: None,
//                             errors: Some(error.to_string()),
//                         });
//                         (StatusCode::INTERNAL_SERVER_ERROR, Json(json_response))
//                     }
//                 }
//             } else {
//                 println!("Error While Adding User Messing fild:{:?}", payload);
//                 let json_response = json!(ApiResponse {
//                     status: "error".to_string(),
//                     code: 500,
//                     message: "messing filds".to_string(),
//                     data: None,
//                     errors: Some(format!("Messing fild:{:?}", payload)),
//                 });
//                 (StatusCode::INTERNAL_SERVER_ERROR, Json(json_response))
//             }
//         }
//         Err(error) => {
//             println!("DB Cannation Error:{}", error);
//             let json_response = json!(ApiResponse {
//                 status: "error".to_string(),
//                 code: 500,
//                 message: "Internal server error".to_string(),
//                 data: None,
//                 errors: Some(error.to_string()),
//             });
//             (StatusCode::INTERNAL_SERVER_ERROR, Json(json_response))
//         }
//     }
// }
// pub async fn get_user(Path(params): Path<String>) -> (StatusCode, Json<Value>) {
//     let db = db::connect_db().await;
//     match db {
//         Ok(db) => {
//             let coll = db.collection::<Document>("users");
//             let oid = ObjectId::parse_str(&params).map_err(|e| e);
//             match oid {
//                 Ok(oid) => {
//                     let res = coll.find_one(doc! {"_id":oid}).await;
//                     match res {
//                         Ok(res) => match res {
//                             Some(data) => {
//                                 println!("User Details:{:?}", data);
//                                 let json_response = json!(ApiResponse {
//                                     status: "Success".to_string(),
//                                     code: 200,
//                                     message: "User retrieved successfully".to_string(),
//                                     data: Some(data),
//                                     errors: None,
//                                 });
//                                 (StatusCode::OK, Json(json_response))
//                             }
//                             None => {
//                                 println!("User Not found orCan Not Get User with Id:{}", params);
//                                 let json_response = json!(ApiResponse {
//                                     status: "error".to_string(),
//                                     code: 404,
//                                     message: "User Not Found".to_string(),
//                                     data: None,
//                                     errors: Some(format!(
//                                         "User Not found or Can Not Get User with Id:{}",
//                                         params
//                                     )),
//                                 });
//                                 (StatusCode::NOT_FOUND, Json(json_response))
//                             }
//                         },
//                         Err(error) => {
//                             println!("Error While getting user:{error}");
//                             let json_response = serde_json::json!(ApiResponse {
//                                 status: "error".to_string(),
//                                 code: 500,
//                                 message: "Internal server error".to_string(),
//                                 data: None,
//                                 errors: Some(error.to_string()),
//                             });
//                             (StatusCode::INTERNAL_SERVER_ERROR, Json(json_response))
//                         }
//                     }
//                 }
//                 Err(error) => {
//                     println!("Invalid Id Formate:{}", error);
//                     let json_response = serde_json::json!(ApiResponse {
//                         status: "error".to_string(),
//                         code: 400,
//                         message: "Invalid ID format".to_string(),
//                         data: None,
//                         errors: Some("Invalid ID format".to_string()),
//                     });
//                     (StatusCode::NOT_FOUND, Json(json_response))
//                 }
//             }
//         }
//         Err(error) => {
//             println!("DB Cannation Error:{}", error);
//             let json_response = serde_json::json!(ApiResponse {
//                 status: "error".to_string(),
//                 code: 500,
//                 message: "Database connection failed".to_string(),
//                 data: None,
//                 errors: Some(error.to_string()),
//             });
//             (StatusCode::INTERNAL_SERVER_ERROR, Json(json_response))
//         }
//     }
// }

// pub async fn update_user(
//     Path(params): Path<String>,
//     Json(payload): Json<User>,
// ) -> (StatusCode, Json<Value>) {
//     let db = db::connect_db().await;
//     match db {
//         Ok(db) => {
//             let coll = db.collection::<User>("users");
//             let oid = ObjectId::parse_str(&params).map_err(|e| e);
//             match oid {
//                 Ok(oid) => {
//                     let mut update_doc = doc! {};

//                     if let Some(first_name) = &payload.first_name {
//                         update_doc.insert("firstName", first_name);
//                     }
//                     if let Some(last_name) = &payload.last_name {
//                         update_doc.insert("lastName", last_name);
//                     }
//                     if let Some(email) = &payload.email {
//                         update_doc.insert("email", email);
//                     }
//                     if let Some(password) = &payload.password {
//                         update_doc.insert("password", password);
//                     }

//                     if !update_doc.is_empty() {
//                         update_doc.insert("updatedAt", DateTime::now());
//                         let update_doc = doc! { "$set": update_doc };
//                         match coll.update_one(doc! {"_id": oid}, update_doc).await {
//                             Ok(res) => {
//                                 println!(
//                                     "Matched {} document(s) and modified {} document(s)",
//                                     res.matched_count, res.modified_count
//                                 );
//                                 let json_response = json!(ApiResponse {
//                                     status: "Success".to_string(),
//                                     code: 200,
//                                     message: "User Added Successfully".to_string(),
//                                     data: Some(format!(
//                                         "Matched {} document(s) and modified {} document(s)",
//                                         res.matched_count, res.modified_count
//                                     )),
//                                     errors: None,
//                                 });
//                                 (StatusCode::OK, Json(json_response))
//                             }
//                             Err(error) => {
//                                 println!("Error while updating user: {}", error);
//                                 let json_response = json!(ApiResponse {
//                                     status: "error".to_string(),
//                                     code: 500,
//                                     message: "Internal server error".to_string(),
//                                     data: None,
//                                     errors: Some(error.to_string()),
//                                 });
//                                 (StatusCode::INTERNAL_SERVER_ERROR, Json(json_response))
//                             }
//                         }
//                     } else {
//                         println!("No such fields to update");
//                         let json_response = json!(ApiResponse {
//                             status: "error".to_string(),
//                             code: 404,
//                             message: "No such fields to update".to_string(),
//                             data: None,
//                             errors: Some(format!("No such fields to update:{:?}", payload)),
//                         });
//                         (StatusCode::NOT_FOUND, Json(json_response))
//                     }
//                 }
//                 Err(error) => {
//                     println!("Invalid Id Formate:{}", error);
//                     let json_response = serde_json::json!(ApiResponse {
//                         status: "error".to_string(),
//                         code: 400,
//                         message: "Invalid ID format".to_string(),
//                         data: None,
//                         errors: Some("Invalid ID format".to_string()),
//                     });
//                     (StatusCode::NOT_FOUND, Json(json_response))
//                 }
//             }
//         }
//         Err(error) => {
//             println!("DB Cannation Error:{}", error);
//             let json_response = json!(ApiResponse {
//                 status: "error".to_string(),
//                 code: 500,
//                 message: "Somthig is Worng".to_string(),
//                 data: None,
//                 errors: Some(format!("Somthig is Worng")),
//             });
//             (StatusCode::INTERNAL_SERVER_ERROR, Json(json_response))
//         }
//     }
// }
// pub async fn delete_user(Path(params): Path<String>) -> (StatusCode, Json<Value>) {
//     let db = db::connect_db().await;
//     match db {
//         Ok(db) => {
//             let coll = db.collection::<User>("users");
//             let oid = ObjectId::parse_str(&params).map_err(|e| e);
//             match oid {
//                 Ok(oid) => {
//                     let delete_user = coll.delete_one(doc! {"_id":oid}).await;
//                     match delete_user {
//                         Ok(res) => {
//                             let json_response = json!(ApiResponse {
//                                 status: "Success".to_string(),
//                                 code: 200,
//                                 message: "User Deleted Successfully".to_string(),
//                                 data: Some(format!(
//                                     "{} User Deleted Successfully",
//                                     res.deleted_count
//                                 )),
//                                 errors: None,
//                             });
//                             (StatusCode::OK, Json(json_response))
//                         }
//                         Err(error) => {
//                             println!(
//                                 "Error While Deleting User with Id:{}, Error:{}",
//                                 params, error
//                             );
//                             let json_response = json!(ApiResponse {
//                                 status: "error".to_string(),
//                                 code: 404,
//                                 message: "Error While Deleting User".to_string(),
//                                 data: None,
//                                 errors: Some(format!("Error While Deleting User")),
//                             });
//                             (StatusCode::NOT_FOUND, Json(json_response))
//                         }
//                     }
//                 }
//                 Err(error) => {
//                     println!("Invalid Id Formate:{}", error);
//                     let json_response = serde_json::json!(ApiResponse {
//                         status: "error".to_string(),
//                         code: 400,
//                         message: "Invalid ID format".to_string(),
//                         data: None,
//                         errors: Some("Invalid ID format".to_string()),
//                     });
//                     (StatusCode::NOT_FOUND, Json(json_response))
//                 }
//             }
//         }
//         Err(error) => {
//             println!("DB Cannation Error:{}", error);
//             let json_response = json!(ApiResponse {
//                 status: "error".to_string(),
//                 code: 500,
//                 message: "Somthig is Worng".to_string(),
//                 data: None,
//                 errors: Some(format!("Somthig is Worng")),
//             });
//             (StatusCode::INTERNAL_SERVER_ERROR, Json(json_response))
//         }
//     }
// }
