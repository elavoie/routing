// Copyright 2015 MaidSafe.net limited
// This MaidSafe Software is licensed to you under (1) the MaidSafe.net Commercial License,
// version 1.0 or later, or (2) The General Public License (GPL), version 3, depending on which
// licence you accepted on initial access to the Software (the "Licences").
// By contributing code to the MaidSafe Software, or to this project generally, you agree to be
// bound by the terms of the MaidSafe Contributor Agreement, version 1.0, found in the root
// directory of this project at LICENSE, COPYING and CONTRIBUTOR respectively and also
// available at: http://www.maidsafe.net/licenses
// Unless required by applicable law or agreed to in writing, the MaidSafe Software distributed
// under the GPL Licence is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS
// OF ANY KIND, either express or implied.
// See the Licences for the specific language governing permissions and limitations relating to
// use of the MaidSafe
// Software.

use types::{Authority, DhtId, DestinationAddress};
use super::{Action, RoutingError};

pub trait Facade : Sync + Send {
  /// if reply is data then we send back the response message (ie get_response )
  fn handle_get(&mut self,
                type_id: u64,
                our_authority: Authority,
                from_authority: Authority,
                from_address: DhtId,
                data: Vec<u8>)
      -> Result<Action, RoutingError>;

  /// data: Vec<u8> is serialised maidsafe_types::Payload which holds typetag and content
  fn handle_put(&mut self,
                our_authority: Authority,
                from_authority: Authority,
                from_address: DhtId,
                dest_address: DestinationAddress,
                data: Vec<u8>)
      -> Result<Action, RoutingError>;

  fn handle_post(&mut self,
                 our_authority: Authority,
                 from_authority: Authority,
                 from_address: DhtId,
                 data: Vec<u8>)
      -> Result<Action, RoutingError>;

  fn handle_get_response(&mut self,
                         from_address: DhtId,
                         response: Result<Vec<u8>,
                         RoutingError>);

  fn handle_put_response(&mut self,
                         from_authority: Authority,
                         from_address: DhtId,
                         response: Result<Vec<u8>,
                         RoutingError>);

  fn handle_post_response(&mut self,
                          from_authority: Authority,
                          from_address: DhtId,
                          response: Result<Vec<u8>,
                          RoutingError>);

  fn add_node(&mut self, node: DhtId);
  fn drop_node(&mut self, node: DhtId);
}
#[test]
fn dummy()  {
}
