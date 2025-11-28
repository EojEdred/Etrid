import requests

class ComplianceAdapter:
    def __init__(self, reserve_api):
        self.reserve_api = reserve_api

    def fetch_reserves(self):
        r = requests.get(self.reserve_api)
        return r.json()

    def kyc_check(self, user_id):
        # TODO: integrate with 3rd-party KYC/AML provider
        return True
