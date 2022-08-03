// KILT Blockchain – https://botlabs.org
// Copyright (C) 2019-2022 BOTLabs GmbH

// The KILT Blockchain is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The KILT Blockchain is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

// If you feel like getting in touch with us, you can do so at info@botlabs.org

use serde::{Deserialize, Serialize};

use kilt_support::deposit::Deposit;
use public_credentials::CredentialEntry;

use public_credentials_rpc::PublicCredentialsFilter;

#[derive(Serialize, Deserialize)]
#[serde(bound(
	serialize = "
	CTypeHash: Serialize,
	Attester: Serialize,
	BlockNumber: Serialize,
	AccountId: Serialize,
	Balance: std::fmt::Display,
	AuthorizationId: Serialize",
	deserialize = "
	CTypeHash: Deserialize<'de>,
	Attester: Deserialize<'de>,
	BlockNumber: Deserialize<'de>,
	AccountId: Deserialize<'de>,
	Balance:  std::str::FromStr,
	AuthorizationId: Deserialize<'de>"
))]
/// Thin wrapper around a runtime credential entry as specified in the
/// `public-credentials` pallet. This wrapper implements all the
/// (de-)serialization logic.
pub struct OuterCredentialEntry<CTypeHash, Attester, BlockNumber, AccountId, Balance, AuthorizationId> {
	pub ctype_hash: CTypeHash,
	pub attester: Attester,
	pub revoked: bool,
	pub block_number: BlockNumber,
	pub deposit: Deposit<AccountId, Balance>,
	pub authorization_id: Option<AuthorizationId>,
}

impl<CTypeHash, Attester, BlockNumber, AccountId, Balance, AuthorizationId>
	From<CredentialEntry<CTypeHash, Attester, BlockNumber, AccountId, Balance, AuthorizationId>>
	for OuterCredentialEntry<CTypeHash, Attester, BlockNumber, AccountId, Balance, AuthorizationId>
{
	fn from(value: CredentialEntry<CTypeHash, Attester, BlockNumber, AccountId, Balance, AuthorizationId>) -> Self {
		Self {
			ctype_hash: value.ctype_hash,
			attester: value.attester,
			revoked: value.revoked,
			block_number: value.block_number,
			deposit: value.deposit,
			authorization_id: value.authorization_id,
		}
	}
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PublicCredentialFilter<CTypeHash, Attester, AuthorizationId> {
	CTypeHash(CTypeHash),
	Attester(Attester),
	AuthorizationId(AuthorizationId),
}

impl<CTypeHash, Attester, BlockNumber, AccountId, Balance, AuthorizationId>
	PublicCredentialsFilter<CredentialEntry<CTypeHash, Attester, BlockNumber, AccountId, Balance, AuthorizationId>>
	for PublicCredentialFilter<CTypeHash, Attester, AuthorizationId>
where
	CTypeHash: Eq,
	Attester: Eq,
	AuthorizationId: Eq,
{
	fn should_include(
		&self,
		credential: &CredentialEntry<CTypeHash, Attester, BlockNumber, AccountId, Balance, AuthorizationId>,
	) -> bool {
		match self {
			Self::CTypeHash(ctype_hash) => ctype_hash == &credential.ctype_hash,
			Self::Attester(attester) => attester == &credential.attester,
			Self::AuthorizationId(authorization_id) => Some(authorization_id) == credential.authorization_id.as_ref(),
		}
	}
}
