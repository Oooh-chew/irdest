import ApplicationSerializer from './application';
import { underscore } from '@ember/string';

export default class ChatMessageSerializer extends ApplicationSerializer {
  normalize (typeClass, hash) {
    hash.kind = 'text';

    if(hash?.room?.Create) {
      hash.room = hash.room.Create.id;
      hash.kind = 'create';
    }

    if(hash?.room?.Id) {
      hash.room = hash.room.Id;
    }

    return super.normalize(...arguments);
  }
  // normalizeSingleResponse (store, primaryModelClass, payload, id, requestType) {

  //   payload[underscore(primaryModelClass.modelName)] = payload[underscore(primaryModelClass.modelName)][0];

  //   payload[underscore(primaryModelClass.modelName)].room = payload[underscore(primaryModelClass.modelName)].room.Id;

  //   return super.normalizeSingleResponse(store, primaryModelClass, payload, id, requestType);
  // }

  // normalizeArrayResponse(store, primaryModelClass, payload, id, requestType) {
  //   const chatMessages = payload.chat_message.filter(x => x.room.Id);
  //   chatMessages.forEach(msg => msg.room = msg.room.Id);
  //   const serialized = super.normalizeArrayResponse(store, primaryModelClass, { chat_messages: chatMessages }, id, requestType);
  //   return serialized;
  // }
}
