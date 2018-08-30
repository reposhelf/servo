# coding=utf-8
#####################################################
# THIS FILE IS AUTOMATICALLY GENERATED. DO NOT EDIT #
#####################################################
# noqa: E128,E201
from .client import BaseClient
from .client import createApiClient
from .client import config
from .client import createTemporaryCredentials
from .client import createSession
_defaultConfig = config


class PurgeCache(BaseClient):
    """
    The purge-cache service, typically available at
    `purge-cache.taskcluster.net`, is responsible for publishing a pulse
    message for workers, so they can purge cache upon request.

    This document describes the API end-point for publishing the pulse
    message. This is mainly intended to be used by tools.
    """

    classOptions = {
        "baseUrl": "https://purge-cache.taskcluster.net/v1/"
    }

    def ping(self, *args, **kwargs):
        """
        Ping Server

        Respond without doing anything.
        This endpoint is used to check that the service is up.

        This method is ``stable``
        """

        return self._makeApiCall(self.funcinfo["ping"], *args, **kwargs)

    def purgeCache(self, *args, **kwargs):
        """
        Purge Worker Cache

        Publish a purge-cache message to purge caches named `cacheName` with
        `provisionerId` and `workerType` in the routing-key. Workers should
        be listening for this message and purge caches when they see it.

        This method takes input: ``v1/purge-cache-request.json#``

        This method is ``stable``
        """

        return self._makeApiCall(self.funcinfo["purgeCache"], *args, **kwargs)

    def allPurgeRequests(self, *args, **kwargs):
        """
        All Open Purge Requests

        This is useful mostly for administors to view
        the set of open purge requests. It should not
        be used by workers. They should use the purgeRequests
        endpoint that is specific to their workerType and
        provisionerId.

        This method gives output: ``v1/all-purge-cache-request-list.json#``

        This method is ``stable``
        """

        return self._makeApiCall(self.funcinfo["allPurgeRequests"], *args, **kwargs)

    def purgeRequests(self, *args, **kwargs):
        """
        Open Purge Requests for a provisionerId/workerType pair

        List of caches that need to be purged if they are from before
        a certain time. This is safe to be used in automation from
        workers.

        This method gives output: ``v1/purge-cache-request-list.json#``

        This method is ``stable``
        """

        return self._makeApiCall(self.funcinfo["purgeRequests"], *args, **kwargs)

    funcinfo = {
        "allPurgeRequests": {
            'args': [],
            'method': 'get',
            'name': 'allPurgeRequests',
            'output': 'v1/all-purge-cache-request-list.json#',
            'query': ['continuationToken', 'limit'],
            'route': '/purge-cache/list',
            'stability': 'stable',
        },
        "ping": {
            'args': [],
            'method': 'get',
            'name': 'ping',
            'route': '/ping',
            'stability': 'stable',
        },
        "purgeCache": {
            'args': ['provisionerId', 'workerType'],
            'input': 'v1/purge-cache-request.json#',
            'method': 'post',
            'name': 'purgeCache',
            'route': '/purge-cache/<provisionerId>/<workerType>',
            'stability': 'stable',
        },
        "purgeRequests": {
            'args': ['provisionerId', 'workerType'],
            'method': 'get',
            'name': 'purgeRequests',
            'output': 'v1/purge-cache-request-list.json#',
            'query': ['since'],
            'route': '/purge-cache/<provisionerId>/<workerType>',
            'stability': 'stable',
        },
    }


__all__ = ['createTemporaryCredentials', 'config', '_defaultConfig', 'createApiClient', 'createSession', 'PurgeCache']
