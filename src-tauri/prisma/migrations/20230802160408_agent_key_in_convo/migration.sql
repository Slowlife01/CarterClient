/*
  Warnings:

  - You are about to drop the column `agentId` on the `Conversation` table. All the data in the column will be lost.
  - Added the required column `agentKey` to the `Conversation` table without a default value. This is not possible if the table is not empty.

*/
-- RedefineTables
PRAGMA foreign_keys=OFF;
CREATE TABLE "new_Conversation" (
    "title" TEXT NOT NULL,
    "id" TEXT NOT NULL,
    "createdAt" DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updatedAt" DATETIME NOT NULL,
    "agentKey" TEXT NOT NULL,
    CONSTRAINT "Conversation_agentKey_fkey" FOREIGN KEY ("agentKey") REFERENCES "Agent" ("key") ON DELETE CASCADE ON UPDATE CASCADE
);
INSERT INTO "new_Conversation" ("createdAt", "id", "title", "updatedAt") SELECT "createdAt", "id", "title", "updatedAt" FROM "Conversation";
DROP TABLE "Conversation";
ALTER TABLE "new_Conversation" RENAME TO "Conversation";
CREATE UNIQUE INDEX "Conversation_id_key" ON "Conversation"("id");
PRAGMA foreign_key_check;
PRAGMA foreign_keys=ON;
