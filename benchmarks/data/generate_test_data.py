import json
import random
import os
import zipfile
from faker import Faker

def generate_github_events(num_events=1000):
    fake = Faker()
    events = []
    
    for _ in range(num_events):
        event = {
            "id": fake.uuid4(),
            "type": random.choice(["PushEvent", "PullRequestEvent", "IssuesEvent"]),
            "actor": {
                "id": fake.random_int(min=1, max=1000000),
                "login": fake.user_name(),
                "display_login": fake.name(),
                "gravatar_id": "",
                "url": fake.url(),
                "avatar_url": fake.image_url()
            },
            "repo": {
                "id": fake.random_int(min=1, max=1000000),
                "name": f"{fake.user_name()}/{fake.word()}",
                "url": fake.url()
            },
            "payload": {
                "push_id": fake.random_int(min=1, max=1000000),
                "size": fake.random_int(min=1, max=10),
                "distinct_size": fake.random_int(min=1, max=10)
            },
            "public": fake.boolean(),
            "created_at": fake.iso8601()
        }
        events.append(event)
    
    return events

def create_test_data(output_dir='test_data', sizes=[10, 100, 1000, 10000]):
    os.makedirs(output_dir, exist_ok=True)
    
    for size in sizes:
        filename = os.path.join(output_dir, f'github_events_{size}.json')
        events = generate_github_events(size)
        
        with open(filename, 'w') as f:
            json.dump(events, f)
        
        # Create zip file
        with zipfile.ZipFile(f'{filename}.zip', 'w') as zipf:
            zipf.write(filename, arcname=os.path.basename(filename))

if __name__ == '__main__':
    create_test_data()
