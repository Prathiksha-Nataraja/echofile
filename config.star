# DEFAULT = ""

# def tasks():
#     def task(kind='',
#              name='',
#              ):
# tasks

# def flows():
#     print("")
# flows

# def workflow():
#     print("")
# workflow

def tasks(kind, name, input_args=None, output_args=None, properties=None):

    return {
        "kind": kind,
        "name": name,
        "input_args": input_args or [],
        "output_args": output_args or [],
        "properties": properties or {},
    }

def flows(type, task_name, depends_on=None):
 
    return {
        "type": type,
        "task_name": task_name,
        "depends_on": depends_on or [],
    }

def workflow(name, version):
  
    return {
        "name": name,
        "version": version,
    }

def my_task():
    return tasks(
        kind="MY_TASK_KIND",
        name="my-task",
        input_args=[
            {"name": "FIELD_NAME", "type": "TYPE"},
        ],
        output_args=[
            {"name": "FIELD_NAME", "type": "TYPE"},
        ],
        properties={"key": "value"},
    )

def my_flows():

    return flows(
        type="FLOW-TYPE",
        task_name="Task-name",
        depends_on=[
            {"operation": "KIND_OF_OPERATION", "task":{"name": "TASK_NAME", "fields": "Task_fields_which_are_Dependent"}},
        ]
    )

def workflows():
    return workflow(
        name="NAME",
        version="VERSION"
    )

