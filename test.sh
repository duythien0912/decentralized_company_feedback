source neardev/dev-account.env

# near call $CONTRACT_NAME set_white_lists '{"white_list": true}' --accountId $CONTRACT_NAME

# near call $CONTRACT_NAME get_all_white_lists '{"from_index": 0, "limit": 10}' --accountId $CONTRACT_NAME

near view $CONTRACT_NAME get_min_user_cap '{}' --accountId $CONTRACT_NAME

near call $CONTRACT_NAME set_min_user_cap '{"min_user_cap": 10000}' --accountId $CONTRACT_NAME

near view $CONTRACT_NAME get_min_user_cap '{}' --accountId $CONTRACT_NAME

near view $CONTRACT_NAME get_owner_id '{}' --accountId $CONTRACT_NAME

near view $CONTRACT_NAME get_owner_id '{}' --accountId $CONTRACT_NAME
near view $CONTRACT_NAME get_min_user_cap '{}' --accountId $CONTRACT_NAME
near view $CONTRACT_NAME get_max_user_cap '{}' --accountId $CONTRACT_NAME
near view $CONTRACT_NAME get_total_allocation '{}' --accountId $CONTRACT_NAME
near view $CONTRACT_NAME get_current_allocation '{}' --accountId $CONTRACT_NAME
near view $CONTRACT_NAME get_open_time '{}' --accountId $CONTRACT_NAME
near view $CONTRACT_NAME get_close_time '{}' --accountId $CONTRACT_NAME
near view $CONTRACT_NAME get_unlock_time '{}' --accountId $CONTRACT_NAME
