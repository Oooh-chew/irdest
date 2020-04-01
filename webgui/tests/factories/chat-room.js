import FactoryGuy from 'ember-data-factory-guy';
import faker from 'faker';

FactoryGuy.define('chat-room', {
  default: {
    multiuser: () => faker.random.boolean(),
    name: () => faker.name.findName(),
    avatar: () => faker.image.avatar(),

    users: () => FactoryGuy.hasMany('user', 2),
    messages: () => FactoryGuy.hasMany('chat-message', 10),
  }
});