import { defineStore } from "pinia";

import type { Poll } from "../poll-types";
import { Socket } from "socket.io";
import { getTokenPayload } from "../utils";
import { nanoid } from "nanoid";

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
  poll: Poll | null;
  accessToken?: string;
  socket?: Socket;
  wsErrors: WsErrorUnique[];
  me: Me | null;
  isLoading: boolean;
};

export const usePollStore = defineStore("PollStore", {
  state: (): StateShape => ({
    poll: null,
    me: null,
    isLoading: false,
    wsErrors: [],
  }),

  getters: {
    getPoll: (state): Poll | null => {
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
      if (!token) {
        return;
      }
      this.accessToken = token;
      const accessToken = getTokenPayload(token);
      this.me = {
        id: accessToken.sub,
        name: accessToken.name,
      };
    },
    initializePoll(poll: Poll) {
      this.poll = poll;
    },
    updatePoll(poll: Poll) {
      this.poll = poll;
    },
    deletePoll() {
      this.poll = null;
    },
    reset() {
      this.poll = null;
      this.accessToken = undefined;
      this.socket = undefined;
      this.wsErrors = [];
    },

    startVote() {
      const poll = this.poll!;
      poll.hasStarted = true;
      this.poll = poll;
    },

    removeParticipant(_id: string) {},

    removeNomination(_id: string) {},

    nominate(text: string) {
      const poll = this.poll!;
      const id = nanoid();
      poll.nominations[id] = {
        userId: this.me?.id!,
        text,
      };
      this.poll = poll;
    },

    cancelPoll() {},

    submitRankings(_rankings: string[]) {},
  },
});
