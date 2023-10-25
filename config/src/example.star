
task(
    "polkadot", "stakingpayout", 
     {"url": "String", 
    "owner_key":"String", 
    "address":"String", 
    "era":"u32"},
    {
        "chain" : "westend",
        "operation" : "stakingpayout"
    },
    ""
    # {
    #     "operation" : "",
    #     "task" : {
    #         "stakingpayout" : "result"
    #     },

    # },
)