import { createApiClient } from './apiClient';

const BASE_URL = process.env.NODE_ENV === 'development'
  ? 'http://localhost:5152'
  : '';

export const atmosApi = createApiClient(BASE_URL);
