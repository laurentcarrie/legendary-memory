from fhirclient import client
from fhirclient.models.encounter import Encounter
from fhirclient.models.procedure import Procedure

settings = {
    'app_id': 'my_web_app',
    'api_base': 'https://r4.smarthealthit.orgxxx'
}
smart = client.FHIRClient(settings=settings)

search = Encounter.where(struct={'subject': '2cda5aad-e409-4070-9a15-e1c35c46ed5a', 'status': 'finished'})
encounters = search.perform_resources(smart.server)
print({res.type[0].text for res in search.perform_resources(smart.server)})
# {'Encounter for symptom', 'Encounter for check up (procedure)'}

# to include the resources referred to by the encounter via `subject` in the results
search = search.include('subject')
print({res.resource_type for res in search.perform_resources(smart.server)})
# {'Encounter', 'Patient'}

# to include the Procedure resources which refer to the encounter via `encounter`
search = search.include('encounter', Procedure, reverse=True)
print({res.resource_type for res in search.perform_resources(smart.server)})
# {'Encounter', 'Patient', 'Procedure'}

# to get the raw Bundle instead of resources only, you can use:
bundle = search.perform(smart.server)
print({entry.resource.resource_type for entry in bundle.entry})
# {'Encounter', 'Patient', 'Procedure'}
