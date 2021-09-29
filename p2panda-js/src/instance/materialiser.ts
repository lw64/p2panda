// SPDX-License-Identifier: AGPL-3.0-or-later

import debug from 'debug';

import { EntryRecord, InstanceRecord } from '~/types';

const log = debug('p2panda-js:message');

/**
 * Create a record of data instances by parsing a series of p2panda log entries
 *
 * @param entries entry records from node
 * @returns records of the instance's data and metadata
 */
export const materializeEntries = (
  entries: EntryRecord[],
): { [instanceId: string]: InstanceRecord } => {
  const instances: { [instanceId: string]: InstanceRecord } = {};
  entries.sort((a, b) => a.seqNum - b.seqNum);
  log(`Materialising ${entries.length} entries`);
  for (const entry of entries) {
    if (entry.message == null) continue;

    const entryHash = entry.encoded.entryHash;
    const author = entry.encoded.author;
    const schema = entry.message.schema;

    if (instances[entryHash] && instances[entryHash].deleted) continue;

    let updated: InstanceRecord;
    switch (entry.message.action) {
      case 'create':
        instances[entryHash] = {
          ...entry.message.fields,
          _meta: {
            author,
            deleted: false,
            edited: false,
            entries: [entry],
            hash: entryHash,
            schema,
          },
        };
        break;

      case 'update':
        updated = {
          ...instances[entryHash],
          ...entry.message.fields,
        };
        updated._meta.edited = true;
        updated._meta.entries.push(entry);
        instances[entryHash] = updated;
        break;

      case 'delete':
        updated = { _meta: instances[entryHash]._meta };
        updated._meta.deleted = true;
        updated._meta.entries.push(entry);
        instances[entryHash] = updated;
        break;
      default:
        throw new Error('Unhandled mesage action');
    }
  }
  log(`Materialisation yields ${Object.keys(instances).length} instances`);
  return instances;
};