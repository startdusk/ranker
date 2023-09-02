import { defineStore } from "pinia";

import type { Poll } from "../poll-types";
import { Socket } from "socket.io";

type Me = {
  id: string;
  name: string;
};

type WsError = {
  type: string;
  message: string;
};

type WsErrorUnique = WsError & {
  id: string;
};

type StateShape = {
  poll?: Poll;
  accessToken?: string;
  socket?: Socket;
  wsErrors: WsErrorUnique[];
  me?: Me;
  isLoading: boolean;
};

export const usePollStore = defineStore("PollStore", {
  state: (): StateShape => ({
    isLoading: false,
    wsErrors: [],
  }),

  getters: {
    getPoll: (state): Poll | undefined => {
      return state.poll;
    },
    isAdmin: (state): boolean => {
      if (!state.me) {
        return false;
      }
      return state.me?.id === state.poll?.adminId;
    },
    participant: (state): string => {
      return state.poll?.participants[state.poll.adminId] || "";
    },
    participantCount: (state): number => {
      return Object.keys(state.poll?.participants || {}).length;
    },
    nominationCount: (state): number => {
      return Object.keys(state.poll?.nominations || {}).length;
    },
    canStartVote: (state): boolean => {
      const votesPerVoter = state.poll?.votesPerVoter ?? 100;
      const nominationCount = Object.keys(state.poll?.nominations || {}).length;
      return nominationCount >= votesPerVoter;
    },
    hasVoted: (state): boolean => {
      const rankings = state.poll?.rankings || {};
      const userID = state.me?.id || "";
      return rankings[userID] !== undefined ? true : false;
    },
    rankingsCount: (state): number => {
      return Object.keys(state.poll?.rankings || {}).length;
    },
  },

  actions: {
    startLoading() {
      this.isLoading = true;
    },
    stopLoading() {
      this.isLoading = false;
    },
    setPollAccessToken(token?: string) {
      this.accessToken = token;
    },
    initializePoll(poll?: Poll) {
      this.poll = poll;
    },
    updatePoll(poll: Poll) {
      this.poll = poll;
    },
    deletePoll() {
      this.poll = undefined;
    },
    reset() {
      this.poll = undefined;
      this.accessToken = undefined;
      this.socket = undefined;
      this.wsErrors = [];
    },

    startVote() {},

    removeParticipant(_id: string) {},

    removeNomination(_id: string) {},

    nominate(_text: string) {},
  },
});
