source neardev/dev-account.env

cat "==> Call fn get_feedbacks"
near view $CONTRACT_NAME get_feedbacks '{"page": 0, "size": 10}' --accountId=$CONTRACT_NAME

cat "==> Call fn create_feedback"
near call $CONTRACT_NAME create_feedback '{ "feedback": { "id": 0, "parent_id": 0, "user_id": 0, "company_id": 0, "content": "Mock feedback", "reaction": 4, "rating": 0, "up_vote": 0, "down_vote": 0, "report_vote": 0, "create_at": 0, "update_at": 0, "activate": true } }' --accountId=$CONTRACT_NAME

cat "==> Call fn update_active_feedback"
near call $CONTRACT_NAME update_active_feedback '{ "id": 1, "activate": true }' --accountId=$CONTRACT_NAME

cat "==> Call fn get_feedbacks"
near view $CONTRACT_NAME get_feedbacks '{"page": 0, "size": 10}' --accountId=$CONTRACT_NAME
