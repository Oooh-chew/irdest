import Base from 'ember-simple-auth/authenticators/base';

export default class QaulAuthenticator extends Base {
  async restore(data) {
    const validateTokenResponse = await fetch('/http/auth', {
      headers: {
        Authorization: JSON.stringify(data),
      }
    });
    if(validateTokenResponse.status < 200 || validateTokenResponse.status >= 300) {
      throw "login not valid"
    }

    return data;
  }

  async authenticate(id, pw) {
    const loginResponse = await fetch('/http/auth', {
      method: 'POST',
      body: JSON.stringify({ id, pw }),
    });
    if(loginResponse.status < 200 || loginResponse.status >= 300) {
      throw new Error("error during login" + await loginResponse.text());
    }

    return (await loginResponse.json()).auth;
  }

  async invalidate({ id, token }) {
    await fetch(`/http/auth`, {
      method: 'DELETE',
      headers: {
        Authorization: JSON.stringify({ id, token }),
      }
    });
  }
}
