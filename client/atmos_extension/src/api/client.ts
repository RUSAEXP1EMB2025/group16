import { createApiClient } from './apiClient';

const BASE_URL = process.env.NODE_ENV === 'development'
  ? 'http://localhost:8080'
  : '';

export const atmosApi = createApiClient(BASE_URL);
